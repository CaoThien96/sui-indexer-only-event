use move_binding::package_provider::{ModuleProvider, MoveModuleProvider};
use move_binding::types::ToRustType;
use move_binding::SuiNetwork;

fn main() {
    let provider = MoveModuleProvider::new(SuiNetwork::Mainnet);
    let pkg = provider
        .get_package("0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb")
        .unwrap();
    if let Some(pool) = pkg.module_map.get("pool") {
        for (name, s) in &pool.structs {
            if name.as_str().ends_with("Event") {
                println!("=== {} ===", name);
                for f in &s.fields {
                    if let move_binary_format::normalized::Type::Struct { address, module, name, .. } =
                        &f.type_
                    {
                        println!(
                            "  {}: module={} name={} addr={} rust={}",
                            f.name,
                            module,
                            name,
                            address,
                            f.type_.to_rust_type()
                        );
                    } else {
                        println!("  {}: {}", f.name, f.type_.to_rust_type());
                    }
                }
            }
        }
    }
}
