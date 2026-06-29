#[test_only]
module snip_vault::vault_ops_tests;

use snip_vault::vault::{Self, AdminCap};
use std::unit_test::{assert_eq, destroy};
use sui::coin;
use sui::sui::SUI;
use sui::test_scenario::{Self as ts};

const DEPLOYER: address = @0xAD;

#[test]
fun authorized_public_deposit_updates_balance() {
    let mut scenario = ts::begin(DEPLOYER);
    let mut vault = vault::init_for_testing(scenario.ctx());

    ts::next_tx(&mut scenario, DEPLOYER);
    let coin = coin::mint_for_testing<SUI>(750, scenario.ctx());
    vault::deposit(&mut vault, coin, scenario.ctx());

    assert_eq!(vault::token_balance<SUI>(&vault), 750);

    destroy(vault);
    scenario.end();
}

#[test]
#[expected_failure(abort_code = vault::EUnauthorized)]
fun unauthorized_public_deposit_rejects() {
    let mut scenario = ts::begin(DEPLOYER);
    let mut vault = vault::init_for_testing(scenario.ctx());

    ts::next_tx(&mut scenario, @0xBAD);
    let coin = coin::mint_for_testing<SUI>(100, scenario.ctx());
    vault::deposit(&mut vault, coin, scenario.ctx());

    destroy(vault);
    scenario.end();
}

#[test]
fun authorized_public_withdraw_reduces_balance() {
    let mut scenario = ts::begin(DEPLOYER);
    let mut vault = vault::init_for_testing(scenario.ctx());

    ts::next_tx(&mut scenario, DEPLOYER);
    vault::deposit(
        &mut vault,
        coin::mint_for_testing<SUI>(1_000, scenario.ctx()),
        scenario.ctx(),
    );

    ts::next_tx(&mut scenario, DEPLOYER);
    let out = vault::withdraw<SUI>(&mut vault, 400, scenario.ctx());
    assert_eq!(out.value(), 400);
    coin::burn_for_testing(out);

    assert_eq!(vault::token_balance<SUI>(&vault), 600);

    destroy(vault);
    scenario.end();
}

#[test]
#[expected_failure(abort_code = vault::EUnauthorized)]
fun unauthorized_public_withdraw_rejects() {
    let mut scenario = ts::begin(DEPLOYER);
    let mut vault = vault::init_for_testing(scenario.ctx());

    ts::next_tx(&mut scenario, DEPLOYER);
    vault::deposit(
        &mut vault,
        coin::mint_for_testing<SUI>(500, scenario.ctx()),
        scenario.ctx(),
    );

    ts::next_tx(&mut scenario, @0xBAD);
    let out = vault::withdraw<SUI>(&mut vault, 100, scenario.ctx());
    coin::burn_for_testing(out);

    destroy(vault);
    scenario.end();
}

#[test]
#[expected_failure(abort_code = vault::EWrongPackageVersion)]
fun stale_package_version_rejects_deposit() {
    let mut scenario = ts::begin(DEPLOYER);
    let mut vault = vault::init_for_testing(scenario.ctx());
    vault::set_package_version_for_testing(&mut vault, 0);

    ts::next_tx(&mut scenario, DEPLOYER);
    vault::deposit(
        &mut vault,
        coin::mint_for_testing<SUI>(100, scenario.ctx()),
        scenario.ctx(),
    );

    destroy(vault);
    scenario.end();
}

#[test]
fun sync_package_version_restores_ops() {
    let mut scenario = ts::begin(DEPLOYER);
    let mut vault = vault::init_for_testing(scenario.ctx());
    vault::set_package_version_for_testing(&mut vault, 0);

    ts::next_tx(&mut scenario, DEPLOYER);
    {
        let cap = ts::take_from_sender<AdminCap>(&scenario);
        vault::sync_package_version(&cap, &mut vault);
        ts::return_to_sender(&scenario, cap);
    };

    ts::next_tx(&mut scenario, DEPLOYER);
    vault::deposit(
        &mut vault,
        coin::mint_for_testing<SUI>(200, scenario.ctx()),
        scenario.ctx(),
    );
    assert_eq!(vault::token_balance<SUI>(&vault), 200);

    destroy(vault);
    scenario.end();
}

#[test]
#[expected_failure(abort_code = vault::EInsufficientBalance)]
fun withdraw_without_deposit_aborts() {
    let mut scenario = ts::begin(DEPLOYER);
    let mut vault = vault::init_for_testing(scenario.ctx());

    ts::next_tx(&mut scenario, DEPLOYER);
    let out = vault::withdraw_coin<SUI>(&mut vault, 1, scenario.ctx());
    coin::burn_for_testing(out);

    destroy(vault);
    scenario.end();
}

#[test]
#[expected_failure(abort_code = vault::EInsufficientBalance)]
fun withdraw_more_than_balance_aborts() {
    let mut scenario = ts::begin(DEPLOYER);
    let mut vault = vault::init_for_testing(scenario.ctx());

    ts::next_tx(&mut scenario, DEPLOYER);
    let coin = coin::mint_for_testing<SUI>(100, scenario.ctx());
    vault::deposit_coin(&mut vault, coin);

    ts::next_tx(&mut scenario, DEPLOYER);
    let out = vault::withdraw_coin<SUI>(&mut vault, 200, scenario.ctx());
    coin::burn_for_testing(out);

    destroy(vault);
    scenario.end();
}

#[test]
fun zero_coin_deposit_is_noop() {
    let mut scenario = ts::begin(DEPLOYER);
    let mut vault = vault::init_for_testing(scenario.ctx());

    ts::next_tx(&mut scenario, DEPLOYER);
    let zero = coin::zero<SUI>(scenario.ctx());
    vault::deposit_coin(&mut vault, zero);

    assert_eq!(vault::token_balance<SUI>(&vault), 0);

    destroy(vault);
    scenario.end();
}

#[test]
fun multiple_deposits_accumulate() {
    let mut scenario = ts::begin(DEPLOYER);
    let mut vault = vault::init_for_testing(scenario.ctx());

    ts::next_tx(&mut scenario, DEPLOYER);
    vault::deposit_coin(&mut vault, coin::mint_for_testing<SUI>(300, scenario.ctx()));
    vault::deposit_coin(&mut vault, coin::mint_for_testing<SUI>(700, scenario.ctx()));

    assert_eq!(vault::token_balance<SUI>(&vault), 1_000);

    destroy(vault);
    scenario.end();
}

#[test]
#[expected_failure(abort_code = vault::EPositionAlreadyStored)]
fun duplicate_position_per_pool_aborts() {
    let mut scenario = ts::begin(DEPLOYER);
    let mut vault = vault::init_for_testing(scenario.ctx());

    ts::next_tx(&mut scenario, DEPLOYER);
    let pos1 = vault::new_test_position(scenario.ctx());
    let pool_id = object::id(&pos1);
    vault::store_position_object(&mut vault, pool_id, pos1);
    let pos2 = vault::new_test_position(scenario.ctx());
    vault::store_position_object(&mut vault, pool_id, pos2);

    destroy(vault);
    scenario.end();
}
