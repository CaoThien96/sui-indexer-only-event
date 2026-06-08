use crate::codegen_options::{CodegenOptions, EmitMode};
use crate::package_provider::{ModuleProvider, MoveModuleProvider};
use crate::types::TypeResolver;
use crate::SuiNetwork;
use anyhow::{anyhow, Context};
use move_binary_format::normalized::{Enum, Function, Struct, Type};
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use once_cell::sync::Lazy;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::str::FromStr;
use std::sync::RwLock;

pub static BINDING_REGISTRY: Lazy<RwLock<HashMap<AccountAddress, String>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub struct MoveCodegen;

impl MoveCodegen {
    pub fn expand(
        network: SuiNetwork,
        package: &str,
        package_alias: &str,
        base_path: &str,
        options: CodegenOptions,
    ) -> Result<TokenStream, anyhow::Error> {
        let module_provider = MoveModuleProvider::new(network);
        let package_data = module_provider.get_package(package)?;

        let package_address = AccountAddress::from_str(package)
            .or_else(|_| AccountAddress::from_hex_literal(package))
            .with_context(|| format!("invalid package address '{package}'"))?;

        let mut cache = BINDING_REGISTRY
            .write()
            .map_err(|e| anyhow!("Failed to acquire write lock: {}", e))?;
        cache.insert(
            package_address,
            format!("{base_path}::{package_alias}"),
        );
        drop(cache);

        let type_resolver = TypeResolver::new(
            base_path,
            package_alias,
            package_address,
            &options.linkage,
        );

        let package_ident = Ident::new(package_alias, proc_macro2::Span::call_site());
        let version = package_data.version;

        if options.register_only {
            return Ok(quote! {
                pub mod #package_ident {
                    pub const PACKAGE_VERSION: u64 = #version;
                }
            });
        }

        let module_tokens = package_data
            .module_map
            .iter()
            .filter(|(module_name, _)| options.should_emit_module(module_name))
            .map(|(module_name, module)| {
                let module_emit_mode = options.module_emit_mode(module_name);
                let module_ident = module_ident(module_name);
                let type_origin_table = package_data
                    .type_origin_table
                    .get(module_name)
                    .cloned()
                    .unwrap_or_default();

                let emit_names = compute_emit_names(
                    &module.structs,
                    &module.enums,
                    module_emit_mode,
                );

                let enum_tokens = Self::create_enums(
                    &module.enums,
                    &type_origin_table,
                    &emit_names,
                    package_address,
                    &type_resolver,
                );
                let struct_tokens = Self::create_structs(
                    &module.structs,
                    &type_origin_table,
                    &emit_names,
                    package_address,
                    &type_resolver,
                )?;

                Ok::<_, anyhow::Error>(if struct_tokens.is_empty() && enum_tokens.is_empty() {
                    quote! {}
                } else {
                    let addr_byte_ident = module.address.to_vec();
                    quote! {
                        pub mod #module_ident {
                            use std::str::FromStr;
                            use move_binding_derive::MoveStruct;
                            use move_types::{Address, ObjectId};
                            use crate::parsed_json::serialize_num;
                            pub const PACKAGE_ID: Address = Address::new([#(#addr_byte_ident),*]);
                            pub const MODULE_NAME: &str = #module_name;
                            #(#enum_tokens)*
                            #(#struct_tokens)*
                        }
                    }
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(quote! {
            pub mod #package_ident {
                pub const PACKAGE_VERSION: u64 = #version;
                #(#module_tokens)*
            }
        })
    }

    fn create_structs(
        structs: &BTreeMap<Identifier, Struct>,
        type_origin_ids: &HashMap<String, AccountAddress>,
        emit_names: &HashSet<String>,
        default_address: AccountAddress,
        type_resolver: &TypeResolver,
    ) -> Result<Vec<TokenStream>, anyhow::Error> {
        structs
            .iter()
            .filter(|(name, move_struct)| {
                emit_names.contains(name.as_str()) && move_struct.type_parameters.is_empty()
            })
            .map(|(name, move_struct)| {
                Self::create_struct(
                    name.as_str(),
                    move_struct,
                    type_origin_ids,
                    default_address,
                    type_resolver,
                )
            })
            .collect()
    }

    fn create_struct(
        struct_name: &str,
        move_struct: &Struct,
        type_origin_id: &HashMap<String, AccountAddress>,
        default_address: AccountAddress,
        type_resolver: &TypeResolver,
    ) -> Result<TokenStream, anyhow::Error> {
        let struct_ident = struct_ident(struct_name);
        let field_tokens = move_struct
            .fields
            .iter()
            .map(|field| {
                let field_ident = field_ident(field.name.as_str());
                let rust_type = type_resolver.to_rust_type(&field.type_)?;
                let field_type: syn::Type = syn::parse_str(&rust_type)?;
                let serialize_attr = numeric_serialize_attr(&rust_type);
                Ok(quote! {
                    #serialize_attr
                    pub #field_ident: #field_type,
                })
            })
            .collect::<Result<Vec<_>, anyhow::Error>>()?;

        let origin = type_origin_id
            .get(struct_name)
            .copied()
            .unwrap_or(default_address);
        let addr_byte_ident = origin.to_vec();
        Ok(quote! {
            #[derive(serde::Deserialize, serde::Serialize, Debug, MoveStruct)]
            pub struct #struct_ident {
                #(#field_tokens)*
            }
            impl #struct_ident {
                pub const TYPE_ORIGIN_ID: Address = Address::new([#(#addr_byte_ident),*]);
            }
        })
    }

    fn create_enums(
        enums: &BTreeMap<Identifier, Enum>,
        type_origin_ids: &HashMap<String, AccountAddress>,
        emit_names: &HashSet<String>,
        default_address: AccountAddress,
        type_resolver: &TypeResolver,
    ) -> Vec<TokenStream> {
        enums
            .iter()
            .filter(|(name, _)| emit_names.contains(name.as_str()))
            .map(|(name, move_enum)| {
                Self::create_enum(
                    name.as_str(),
                    move_enum,
                    type_origin_ids,
                    default_address,
                    type_resolver,
                )
            })
            .collect()
    }

    fn create_enum(
        enum_name: &str,
        move_enum: &Enum,
        type_origin_id: &HashMap<String, AccountAddress>,
        default_address: AccountAddress,
        type_resolver: &TypeResolver,
    ) -> TokenStream {
        let enum_ident = struct_ident(enum_name);
        let variant_tokens = move_enum.variants.iter().map(|variant| {
            let variant_ident = field_ident(variant.name.as_str());

            if variant.fields.is_empty() {
                return quote! {#variant_ident,};
            }

            if variant
                .fields
                .iter()
                .enumerate()
                .all(|(i, field)| field.name.to_string() == format!("pos{i}"))
            {
                let field_types = variant.fields.iter().map(|field| {
                    let field_type: syn::Type =
                        syn::parse_str(&type_resolver.to_rust_type(&field.type_).unwrap()).unwrap();
                    quote! {#field_type,}
                });

                return quote! {
                    #variant_ident(#(#field_types)*),
                };
            }

            let field_tokens = variant.fields.iter().map(|field| {
                let field_ident = field_ident(field.name.as_str());
                let field_type: syn::Type =
                    syn::parse_str(&type_resolver.to_rust_type(&field.type_).unwrap()).unwrap();
                quote! {#field_ident: #field_type,}
            });
            quote! { #variant_ident {#(#field_tokens)*},}
        });

        let origin = type_origin_id
            .get(enum_name)
            .copied()
            .unwrap_or(default_address);
        let addr_byte_ident = origin.to_vec();

        quote! {
            #[derive(serde::Deserialize, serde::Serialize, Debug, MoveStruct)]
            pub enum #enum_ident {
                #(#variant_tokens)*
            }

            impl #enum_ident {
                pub const TYPE_ORIGIN_ID: Address = Address::new([#(#addr_byte_ident),*]);
            }
        }
    }

    #[allow(dead_code)]
    fn create_funs(funs: &BTreeMap<Identifier, Function>) -> Vec<TokenStream> {
        funs.iter()
            .flat_map(|(name, fun)| Self::create_fun(name.as_str(), fun))
            .collect()
    }

    #[allow(dead_code)]
    fn create_fun(fun_name: &str, fun: &Function) -> Option<TokenStream> {
        let _ = (fun_name, fun);
        None
    }
}

fn compute_emit_names(
    structs: &BTreeMap<Identifier, Struct>,
    enums: &BTreeMap<Identifier, Enum>,
    emit_mode: EmitMode,
) -> HashSet<String> {
    let mut emit = HashSet::new();

    match emit_mode {
        EmitMode::All | EmitMode::ModuleStructs => {
            for (name, move_struct) in structs {
                if move_struct.type_parameters.is_empty() {
                    emit.insert(name.to_string());
                }
            }
            for name in enums.keys() {
                emit.insert(name.to_string());
            }
            return emit;
        }
        EmitMode::EventStructs | EmitMode::Indexed => {
            for (name, move_struct) in structs {
                if should_emit_struct(name.as_str(), move_struct, emit_mode) {
                    emit.insert(name.to_string());
                }
            }
        }
    }

    let mut queue: VecDeque<String> = emit.iter().cloned().collect();
    while let Some(name) = queue.pop_front() {
        if let Some(move_struct) = structs.get(&Identifier::new(name.as_str()).unwrap()) {
            for field in &move_struct.fields {
                enqueue_local_type_deps(&field.type_, structs, enums, &mut emit, &mut queue);
            }
        }
        if let Some(move_enum) = enums.get(&Identifier::new(name.as_str()).unwrap()) {
            for variant in &move_enum.variants {
                for field in &variant.fields {
                    enqueue_local_type_deps(
                        &field.type_,
                        structs,
                        enums,
                        &mut emit,
                        &mut queue,
                    );
                }
            }
        }
    }

    emit
}

fn enqueue_local_type_deps(
    ty: &Type,
    structs: &BTreeMap<Identifier, Struct>,
    enums: &BTreeMap<Identifier, Enum>,
    emit: &mut HashSet<String>,
    queue: &mut VecDeque<String>,
) {
    match ty {
        Type::Struct { name, .. } => {
            let name = name.to_string();
            let id = Identifier::new(name.as_str()).unwrap();
            if (structs.contains_key(&id) || enums.contains_key(&id)) && emit.insert(name.clone()) {
                queue.push_back(name);
            }
        }
        Type::Vector(inner) => {
            enqueue_local_type_deps(inner, structs, enums, emit, queue);
        }
        Type::Reference(inner) | Type::MutableReference(inner) => {
            enqueue_local_type_deps(inner, structs, enums, emit, queue);
        }
        _ => {}
    }
}

fn should_emit_struct(name: &str, move_struct: &Struct, emit_mode: EmitMode) -> bool {
    if !move_struct.type_parameters.is_empty() {
        return false;
    }

    match emit_mode {
        EmitMode::All => true,
        EmitMode::EventStructs => name.ends_with("Event"),
        EmitMode::ModuleStructs => true,
        EmitMode::Indexed => name.ends_with("Event"),
    }
}

fn numeric_serialize_attr(rust_type: &str) -> TokenStream {
    if matches!(rust_type, "u64" | "u128" | "u32" | "u16" | "u8") {
        quote! { #[serde(serialize_with = "serialize_num")] }
    } else {
        quote! {}
    }
}

fn field_ident(name: &str) -> Ident {
    match name {
        "for" | "ref" => Ident::new(&format!("{name}_"), proc_macro2::Span::call_site()),
        "mod" | "type" | "self" | "crate" => {
            Ident::new_raw(name, proc_macro2::Span::call_site())
        }
        _ => Ident::new(name, proc_macro2::Span::call_site()),
    }
}

fn struct_ident(name: &str) -> Ident {
    Ident::new(name, proc_macro2::Span::call_site())
}

fn module_ident(name: &str) -> Ident {
    if name == "mod" || name == "type" || name == "self" || name == "crate" {
        Ident::new_raw(name, proc_macro2::Span::call_site())
    } else {
        Ident::new(name, proc_macro2::Span::call_site())
    }
}
