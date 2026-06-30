#[test_only]
module snip_vault::acl_tests;

use snip_vault::vault::{Self, AdminCap};
use std::unit_test::{assert_eq, destroy};
use sui::coin;
use sui::sui::SUI;
use sui::test_scenario::{Self as ts};

const DEPLOYER: address = @0xAD;
const BOT: address = @0xB07;
const STRANGER: address = @0xBAD;

#[test]
fun add_bot_and_authorize() {
    let mut scenario = ts::begin(DEPLOYER);
    let mut vault = vault::init_for_testing(scenario.ctx());

    ts::next_tx(&mut scenario, DEPLOYER);
    {
        let cap = ts::take_from_sender<AdminCap>(&scenario);
        vault::add_bot(&cap, &mut vault, BOT);
        ts::return_to_sender(&scenario, cap);
    };

    ts::next_tx(&mut scenario, BOT);
    vault::assert_authorized(&vault, scenario.ctx());

    destroy(vault);
    scenario.end();
}

#[test]
fun deployer_authorized_without_bot() {
    let mut scenario = ts::begin(DEPLOYER);
    let vault = vault::init_for_testing(scenario.ctx());

    ts::next_tx(&mut scenario, DEPLOYER);
    vault::assert_authorized(&vault, scenario.ctx());

    destroy(vault);
    scenario.end();
}

#[test]
#[expected_failure(abort_code = vault::EUnauthorized)]
fun unauthorized_rejected() {
    let mut scenario = ts::begin(DEPLOYER);
    let vault = vault::init_for_testing(scenario.ctx());

    ts::next_tx(&mut scenario, STRANGER);
    vault::assert_authorized(&vault, scenario.ctx());

    destroy(vault);
    scenario.end();
}

#[test]
fun token_balance_reflects_deposits() {
    let mut scenario = ts::begin(DEPLOYER);
    let mut vault = vault::init_for_testing(scenario.ctx());

    assert_eq!(vault::token_balance<SUI>(&vault), 0);

    ts::next_tx(&mut scenario, DEPLOYER);
    let coin = coin::mint_for_testing<SUI>(1_500, scenario.ctx());
    vault::deposit_coin(&mut vault, coin);

    assert_eq!(vault::token_balance<SUI>(&vault), 1_500);

    destroy(vault);
    scenario.end();
}

#[test]
fun deposit_and_withdraw_balance() {
    let mut scenario = ts::begin(DEPLOYER);
    let mut vault = vault::init_for_testing(scenario.ctx());

    ts::next_tx(&mut scenario, DEPLOYER);
    let coin = coin::mint_for_testing<SUI>(1_000, scenario.ctx());
    vault::deposit_coin(&mut vault, coin);

    ts::next_tx(&mut scenario, DEPLOYER);
    let out = vault::withdraw_coin<SUI>(&mut vault, 400, scenario.ctx());
    assert_eq!(out.value(), 400);
    coin::burn_for_testing(out);

    assert_eq!(vault::token_balance<SUI>(&vault), 600);

    destroy(vault);
    scenario.end();
}

#[test]
#[expected_failure(abort_code = vault::EBotAlreadyListed)]
fun add_bot_twice_fails() {
    let mut scenario = ts::begin(DEPLOYER);
    let mut vault = vault::init_for_testing(scenario.ctx());

    ts::next_tx(&mut scenario, DEPLOYER);
    let cap = ts::take_from_sender<AdminCap>(&scenario);
    vault::add_bot(&cap, &mut vault, BOT);
    vault::add_bot(&cap, &mut vault, BOT);

    ts::return_to_sender(&scenario, cap);
    destroy(vault);
    scenario.end();
}

#[test]
#[expected_failure(abort_code = vault::EUnauthorized)]
fun removed_bot_cannot_call() {
    let mut scenario = ts::begin(DEPLOYER);
    let mut vault = vault::init_for_testing(scenario.ctx());

    ts::next_tx(&mut scenario, DEPLOYER);
    {
        let cap = ts::take_from_sender<AdminCap>(&scenario);
        vault::add_bot(&cap, &mut vault, BOT);
        vault::remove_bot(&cap, &mut vault, BOT);
        ts::return_to_sender(&scenario, cap);
    };

    ts::next_tx(&mut scenario, BOT);
    vault::assert_authorized(&vault, scenario.ctx());

    destroy(vault);
    scenario.end();
}
