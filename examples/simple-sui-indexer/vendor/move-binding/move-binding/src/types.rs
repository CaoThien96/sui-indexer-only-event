use crate::move_codegen::BINDING_REGISTRY;
use itertools::Itertools;
use move_binary_format::normalized::Type;
use move_core_types::account_address::AccountAddress;
use std::collections::HashMap;

pub struct TypeResolver {
    base_path: String,
    package_alias: String,
    bindings: HashMap<AccountAddress, String>,
}

impl TypeResolver {
    pub fn new(
        base_path: &str,
        package_alias: &str,
        package_address: AccountAddress,
        linkage: &HashMap<AccountAddress, String>,
    ) -> Self {
        let mut bindings = linkage.clone();
        bindings.insert(package_address, package_alias.to_string());

        if let Ok(cache) = BINDING_REGISTRY.read() {
            for (address, path) in cache.iter() {
                if let Some(alias) = path.rsplit("::").next() {
                    bindings.entry(*address).or_insert_with(|| alias.to_string());
                }
            }
        }

        Self {
            base_path: base_path.to_string(),
            package_alias: package_alias.to_string(),
            bindings,
        }
    }

    pub fn to_rust_type(&self, ty: &Type) -> Result<String, anyhow::Error> {
        Ok(match ty {
            Type::Bool => "bool".to_string(),
            Type::U8 => "u8".to_string(),
            Type::U16 => "u16".to_string(),
            Type::U32 => "u32".to_string(),
            Type::U64 => "u64".to_string(),
            Type::U128 => "u128".to_string(),
            Type::U256 => "move_types::U256".to_string(),
            Type::Address => "Address".to_string(),
            Type::Signer => "Address".to_string(),
            Type::Struct { .. } => self.resolve_struct_type(ty)?,
            Type::Vector(t) => format!("Vec<{}>", self.to_rust_type(t)?),
            Type::Reference(t) => format!("&'static {}", self.to_rust_type(t)?),
            Type::MutableReference(t) => format!("&'static mut {}", self.to_rust_type(t)?),
            Type::TypeParameter(index) => format!("T{index}"),
        })
    }

    fn resolve_struct_type(&self, ty: &Type) -> Result<String, anyhow::Error> {
        let Type::Struct {
            address,
            module,
            name,
            type_arguments,
        } = ty
        else {
            unreachable!()
        };

        match (address, module.as_str(), name.as_str()) {
            (&AccountAddress::ONE, "type_name", "TypeName") => Ok("String".to_string()),
            (&AccountAddress::ONE, "string", "String") => Ok("String".to_string()),
            (&AccountAddress::ONE, "ascii", "String") => Ok("String".to_string()),
            (&AccountAddress::ONE, "option", "Option") => Ok(format!(
                "Option<{}>",
                self.to_rust_type(&type_arguments[0])?
            )),
            (&AccountAddress::TWO, "object", "UID") => Ok("ObjectId".to_string()),
            (&AccountAddress::TWO, "object", "ID") => Ok("ObjectId".to_string()),
            _ => {
                let alias = self.bindings.get(address).ok_or_else(|| {
                    anyhow::anyhow!(
                        "missing linkage for address 0x{address} (type {module}::{name}); \
                         add `linkage = \"0x{address}=ALIAS,...\"` to move_contract!"
                    )
                })?;

                let type_ = format!(
                    "{}::{}::{}::{}",
                    self.base_path,
                    alias,
                    module_path_segment(module.as_str()),
                    name
                );
                if type_arguments.is_empty() {
                    Ok(type_)
                } else {
                    Ok(format!(
                        "{type_}<{}>",
                        type_arguments
                            .iter()
                            .map(|ty| self.to_rust_type(ty))
                            .collect::<Result<Vec<_>, _>>()?
                            .join(", ")
                    ))
                }
            }
        }
    }
}

fn module_path_segment(name: &str) -> String {
    match name {
        "mod" | "type" | "self" | "crate" => format!("r#{name}"),
        _ => name.to_string(),
    }
}

// Legacy trait kept for examples; prefer TypeResolver in codegen.
pub trait ToRustType {
    fn to_rust_type(&self) -> String;
}

impl ToRustType for Type {
    fn to_rust_type(&self) -> String {
        TypeResolver::new("crate", "unknown", AccountAddress::ZERO, &HashMap::new())
            .to_rust_type(self)
            .unwrap_or_else(|_| "Unknown".to_string())
    }
}
