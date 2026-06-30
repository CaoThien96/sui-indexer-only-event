/// Shared snip vault: dynamic token balances (Bag) + LP positions (ObjectBag).
/// Only deployer or allowlisted bot addresses may call snip/sell ops.
module snip_vault::vault;

use std::type_name::{Self, TypeName};
use sui::bag::{Self, Bag};
use sui::balance::{Self, Balance};
use sui::coin::{Self, Coin};
use sui::event;
use sui::object_bag::{Self, ObjectBag};
use sui::table::{Self, Table};

// === Errors ===

#[error]
const EUnauthorized: vector<u8> = b"caller is not deployer or allowlisted bot";
#[error]
const EInsufficientBalance: vector<u8> = b"vault bag balance insufficient for withdrawal";
#[error]
const EBotAlreadyListed: vector<u8> = b"bot address is already on the allowlist";
#[error]
const EBotNotListed: vector<u8> = b"bot address is not on the allowlist";
#[error]
const EPositionAlreadyStored: vector<u8> = b"vault already holds an LP position for this pool";
#[error]
const EWrongPackageVersion: vector<u8> = b"vault package_version does not match this package build; upgrade indexer and call sync_package_version";

/// Bump on each release that must block older published package IDs.
const PACKAGE_VERSION: u64 = 1;

// === Objects ===

/// Deployer capability — transfer to cold wallet after setup.
public struct AdminCap has key, store {
    id: UID,
}

/// Shared vault holding heterogeneous token balances and LP position objects.
public struct Vault has key {
    id: UID,
    deployer: address,
    bots: Table<address, bool>,
    balances: Bag,
    positions: ObjectBag,
    /// Must equal `PACKAGE_VERSION` in the calling package; admin bumps via `sync_package_version` after upgrade.
    package_version: u64,
}

// === Events ===

public struct VaultCreated has copy, drop {
    vault_id: ID,
    deployer: address,
}

public struct BotAdded has copy, drop {
    vault_id: ID,
    bot: address,
}

public struct BotRemoved has copy, drop {
    vault_id: ID,
    bot: address,
}

public struct TokenDeposited has copy, drop {
    vault_id: ID,
    depositor: address,
    coin_type: TypeName,
    amount: u64,
}

public struct TokenWithdrawn has copy, drop {
    vault_id: ID,
    recipient: address,
    coin_type: TypeName,
    amount: u64,
}

public struct PackageVersionSynced has copy, drop {
    vault_id: ID,
    package_version: u64,
}

// === Init ===

fun init(ctx: &mut TxContext) {
    let deployer = ctx.sender();
    let vault = Vault {
        id: object::new(ctx),
        deployer,
        bots: table::new(ctx),
        balances: bag::new(ctx),
        positions: object_bag::new(ctx),
        package_version: PACKAGE_VERSION,
    };
    let vault_id = object::id(&vault);
    transfer::share_object(vault);

    let cap = AdminCap { id: object::new(ctx) };
    transfer::transfer(cap, deployer);

    event::emit(VaultCreated { vault_id, deployer });
}

// === ACL (admin) ===

/// Register a bot wallet that may call snip/sell entrypoints.
public fun add_bot(cap: &AdminCap, vault: &mut Vault, bot: address) {
    assert_package_version(vault);
    assert!(!table::contains(&vault.bots, bot), EBotAlreadyListed);
    table::add(&mut vault.bots, bot, true);
    event::emit(BotAdded { vault_id: object::id(vault), bot });
    cap; // capability witness
}

/// Revoke bot access.
public fun remove_bot(cap: &AdminCap, vault: &mut Vault, bot: address) {
    assert_package_version(vault);
    assert!(table::contains(&vault.bots, bot), EBotNotListed);
    table::remove(&mut vault.bots, bot);
    event::emit(BotRemoved { vault_id: object::id(vault), bot });
    cap;
}

/// After `sui client upgrade`, call once so the vault accepts only the new package build.
public fun sync_package_version(cap: &AdminCap, vault: &mut Vault) {
    vault.package_version = PACKAGE_VERSION;
    event::emit(PackageVersionSynced {
        vault_id: object::id(vault),
        package_version: PACKAGE_VERSION,
    });
    cap;
}

// === ACL (ops) ===

fun assert_package_version(vault: &Vault) {
    assert!(vault.package_version == PACKAGE_VERSION, EWrongPackageVersion);
}

public(package) fun assert_authorized(vault: &Vault, ctx: &TxContext) {
    assert_package_version(vault);
    let sender = ctx.sender();
    assert!(
        sender == vault.deployer || table::contains(&vault.bots, sender),
        EUnauthorized,
    );
}

// === Balance helpers ===

/// Deposit wallet coins into the vault Bag (deployer or allowlisted bot only).
public fun deposit<T>(vault: &mut Vault, coin: Coin<T>, ctx: &TxContext) {
    assert_authorized(vault, ctx);
    let amount = coin.value();
    let coin_type = type_name::with_defining_ids<T>();
    deposit_coin(vault, coin);
    if (amount > 0) {
        event::emit(TokenDeposited {
            vault_id: object::id(vault),
            depositor: ctx.sender(),
            coin_type,
            amount,
        });
    };
}

/// Withdraw tokens from the vault Bag to the caller (deployer or allowlisted bot only).
public fun withdraw<T>(vault: &mut Vault, amount: u64, ctx: &mut TxContext): Coin<T> {
    assert_authorized(vault, ctx);
    let coin_type = type_name::with_defining_ids<T>();
    let coin = withdraw_coin(vault, amount, ctx);
    event::emit(TokenWithdrawn {
        vault_id: object::id(vault),
        recipient: ctx.sender(),
        coin_type,
        amount,
    });
    coin
}

public(package) fun deposit_coin<T>(vault: &mut Vault, coin: Coin<T>) {
    assert_package_version(vault);
    if (coin.value() == 0) {
        coin.destroy_zero();
        return
    };
    let key = type_name::with_defining_ids<T>();
    if (bag::contains(&vault.balances, key)) {
        let bal = bag::borrow_mut<TypeName, Balance<T>>(&mut vault.balances, key);
        bal.join(coin.into_balance());
    } else {
        bag::add(&mut vault.balances, key, coin.into_balance());
    }
}

/// On-chain token balance held in the vault Bag (for off-chain clamp via dev-inspect).
public fun token_balance<T>(vault: &Vault): u64 {
    assert_package_version(vault);
    let key = type_name::with_defining_ids<T>();
    if (!bag::contains(&vault.balances, key)) {
        return 0
    };
    let bal = bag::borrow<TypeName, Balance<T>>(&vault.balances, key);
    bal.value()
}

public(package) fun withdraw_coin<T>(
    vault: &mut Vault,
    amount: u64,
    ctx: &mut TxContext,
): Coin<T> {
    assert_package_version(vault);
    let key = type_name::with_defining_ids<T>();
    assert!(bag::contains(&vault.balances, key), EInsufficientBalance);
    let bal = bag::borrow_mut<TypeName, Balance<T>>(&mut vault.balances, key);
    assert!(bal.value() >= amount, EInsufficientBalance);
    coin::take(bal, amount, ctx)
}

public(package) fun deposit_balance<T>(vault: &mut Vault, bal: Balance<T>) {
    if (bal.value() == 0) {
        bal.destroy_zero();
        return
    };
    let key = type_name::with_defining_ids<T>();
    if (bag::contains(&vault.balances, key)) {
        let existing = bag::borrow_mut<TypeName, Balance<T>>(&mut vault.balances, key);
        existing.join(bal);
    } else {
        bag::add(&mut vault.balances, key, bal);
    }
}

// === Position storage ===

public(package) fun store_position_object<T: key + store>(
    vault: &mut Vault,
    pool_id: ID,
    obj: T,
) {
    assert_package_version(vault);
    assert!(!object_bag::contains(&vault.positions, pool_id), EPositionAlreadyStored);
    object_bag::add(&mut vault.positions, pool_id, obj);
}

#[test_only]
public(package) fun set_package_version_for_testing(vault: &mut Vault, version: u64) {
    vault.package_version = version;
}

#[test_only]
public struct TestPosition has key, store {
    id: UID,
}

#[test_only]
public fun new_test_position(ctx: &mut TxContext): TestPosition {
    TestPosition { id: object::new(ctx) }
}

#[test_only]
public fun init_for_testing(ctx: &mut TxContext): Vault {
    let deployer = ctx.sender();
    let vault = Vault {
        id: object::new(ctx),
        deployer,
        bots: table::new(ctx),
        balances: bag::new(ctx),
        positions: object_bag::new(ctx),
        package_version: PACKAGE_VERSION,
    };
    let cap = AdminCap { id: object::new(ctx) };
    transfer::transfer(cap, deployer);
    vault
}
