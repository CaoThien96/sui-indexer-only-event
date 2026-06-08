# event-bindings-probe — `move_contract!` experiment

Historical probe crate. **Production bindings live in `../event-bindings/`** using the same patched `vendor/move-binding/`.

## Patches in `../vendor/move-binding`

1. **GraphQL URL** — `graphql.mainnet.sui.io` (upstream URL is dead)
2. **Event-only codegen** — `event_modules`, `support_modules`, `emit_mode`
3. **Explicit `linkage`** — cross-package type paths (proc macro order is undefined)
4. **Dependency closure** — emit enums/structs referenced by event fields
5. **No PTB functions** — structs only for BCS decode
6. **Numeric JSON** — `u64`/`u128` serialized as strings for fullnode parity

## Build production crate

```bash
cd ../event-bindings
cargo build    # needs network on first build
cargo test
```

## Linkage deps (Cetus CLMM example)

```rust
move_contract! {
    alias = "integer_mate",
    package = "0xdfaa...",
    emit_mode = "module_structs",
    modules = "i32,i64,i128,...",
    network = "mainnet",
}

move_contract! {
    alias = "pkg_1eab",
    package = "0x1eab...",
    event_modules = "pool,partner,factory",
    linkage = "0x2=sui,0xdfaa...=integer_mate,0x714a...=integer_mate",
    network = "mainnet",
}
```

Multiple defining IDs can map to the same alias when packages were upgraded.
