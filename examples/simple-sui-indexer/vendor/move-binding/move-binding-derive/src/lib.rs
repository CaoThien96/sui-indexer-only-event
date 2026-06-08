use move_binding::move_codegen::MoveCodegen;
use move_binding::CodegenOptions;
use move_binding::SuiNetwork;
use move_core_types::account_address::AccountAddress;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, DeriveInput, ExprPath, GenericParam, Generics, LitBool, LitStr, Token};
use std::collections::HashMap;
use std::str::FromStr;

#[proc_macro_derive(Key)]
pub fn key_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let types = extract_type_ident(&ast.generics);

    let (types_with_trait, types) = if types.is_empty() {
        (quote! {}, quote! {})
    } else {
        (
            quote! {<#(#types:move_types::MoveType),*>},
            quote! {<#(#types),*>},
        )
    };

    let gen = quote! {
        impl #types_with_trait move_types::Key for #name #types {
            fn id(&self) -> &move_types::ObjectId {
                &self.id
            }
        }
    };
    gen.into()
}

fn extract_type_ident(generics: &Generics) -> Vec<Ident> {
    generics
        .params
        .iter()
        .flat_map(|p| {
            if let GenericParam::Type(t) = p {
                Some(t.ident.clone())
            } else {
                None
            }
        })
        .collect()
}

#[proc_macro_derive(MoveStruct)]
pub fn move_struct_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let types = extract_type_ident(&ast.generics);
    let name_str = name.to_string();

    let gen = if types.is_empty() {
        quote! {
            impl move_types::MoveStruct for #name {
                fn struct_type() -> move_types::StructTag {
                    move_types::StructTag {
                        address: Self::TYPE_ORIGIN_ID,
                        module: move_types::Identifier::from_str(MODULE_NAME).unwrap(),
                        name: move_types::Identifier::from_str(#name_str).unwrap(),
                        type_params: vec![],
                    }
                }
            }
        }
    } else {
        quote! {
            impl <#(#types:move_types::MoveType), *> move_types::MoveStruct for #name<#(#types),*> {
                fn struct_type() -> move_types::StructTag {
                    move_types::StructTag {
                        address: Self::TYPE_ORIGIN_ID,
                        module: move_types::Identifier::from_str(MODULE_NAME).unwrap(),
                        name: move_types::Identifier::from_str(#name_str).unwrap(),
                        type_params: vec![#(#types::type_()),*],
                    }
                }
            }
        }
    };
    gen.into()
}

struct MoveContractArgs {
    network: SuiNetwork,
    package_alias: String,
    package: String,
    path: Option<String>,
    options: CodegenOptions,
}

fn parse_csv_modules(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(str::to_string)
        .collect()
}

fn parse_linkage(value: &str) -> Result<HashMap<AccountAddress, String>, syn::Error> {
    let mut linkage = HashMap::new();
    for part in value.split(',').map(str::trim).filter(|part| !part.is_empty()) {
        let (address, alias) = part.split_once('=').ok_or_else(|| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("invalid linkage entry '{part}', expected 0xADDR=alias"),
            )
        })?;
        let parsed = AccountAddress::from_str(address.trim()).map_err(|error| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("invalid linkage address '{address}': {error}"),
            )
        })?;
        linkage.insert(parsed, alias.trim().to_string());
    }
    Ok(linkage)
}

fn parse_emit_mode(value: &str) -> Result<move_binding::EmitMode, syn::Error> {
    match value.to_ascii_lowercase().as_str() {
        "all" => Ok(move_binding::EmitMode::All),
        "event_structs" => Ok(move_binding::EmitMode::EventStructs),
        "module_structs" => Ok(move_binding::EmitMode::ModuleStructs),
        "indexed" => Ok(move_binding::EmitMode::Indexed),
        other => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("Unknown emit_mode '{other}', expected all|event_structs|module_structs|indexed"),
        )),
    }
}

impl Parse for MoveContractArgs {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut alias = None;
        let mut package = None;
        let mut path = None;
        let mut network = SuiNetwork::Mainnet;
        let mut options = CodegenOptions::default();

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            if key == "alias" {
                alias = Some(input.parse::<LitStr>()?.value());
            } else if key == "package" {
                package = Some(input.parse::<LitStr>()?.value());
            } else if key == "base_path" {
                let p = input.parse::<ExprPath>()?.path;
                path = Some(quote!(#p).to_string());
            } else if key == "network" {
                let lit = input.parse::<LitStr>()?;
                network = match lit.value().to_ascii_lowercase().as_str() {
                    "mainnet" => SuiNetwork::Mainnet,
                    "testnet" => SuiNetwork::Testnet,
                    _ => {
                        return Err(syn::Error::new(
                            key.span(),
                            "Unknown network, only ['mainnet', 'testnet'] are supported.",
                        ))
                    }
                };
            } else if key == "register_only" {
                options.register_only = input.parse::<LitBool>()?.value;
            } else if key == "emit_mode" {
                options.emit_mode = parse_emit_mode(&input.parse::<LitStr>()?.value())?;
            } else if key == "modules" {
                options.modules = Some(parse_csv_modules(&input.parse::<LitStr>()?.value()));
            } else if key == "event_modules" {
                options.event_modules = Some(parse_csv_modules(&input.parse::<LitStr>()?.value()));
            } else if key == "support_modules" {
                options.support_modules = Some(parse_csv_modules(&input.parse::<LitStr>()?.value()));
            } else if key == "linkage" {
                options.linkage = parse_linkage(&input.parse::<LitStr>()?.value())?;
            } else {
                return Err(syn::Error::new(key.span(), "Unknown key"));
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(MoveContractArgs {
            network,
            package_alias: alias.ok_or_else(|| syn::Error::new(input.span(), "Missing alias"))?,
            package: package.ok_or_else(|| syn::Error::new(input.span(), "Missing package"))?,
            path,
            options,
        })
    }
}

#[proc_macro]
pub fn move_contract(input: TokenStream) -> TokenStream {
    let MoveContractArgs {
        network,
        package_alias,
        package,
        path,
        options,
    } = parse_macro_input!(input as MoveContractArgs);
    MoveCodegen::expand(
        network,
        &package,
        &package_alias,
        &path.unwrap_or_else(|| "crate".to_string()),
        options,
    )
    .unwrap_or_else(|error| panic!("move_contract! failed for package {package}: {error}"))
    .into()
}
