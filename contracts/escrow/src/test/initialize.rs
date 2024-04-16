#![cfg(test)]

extern crate std;

use crate::events::INIT_TOPIC;
use crate::test::utils::{create_token_contract, native_asset_contract_address};

use crate::{EscrowContract, EscrowContractClient};
use soroban_sdk::{
    testutils::{Address as AddressTestTrait, Events},
    Address, Env, IntoVal,
};

#[test]
fn test_init_with_asset() {
    let env = Env::default();

    let contract_id = env.register_contract(None, EscrowContract);
    let escrow_client = EscrowContractClient::new(&env, &contract_id);

    // let native_assset_address = native_asset_contract_address(&env);
    let admin_address = Address::generate(&env);

    let (asset, _) = create_token_contract(&env, &admin_address);
    let mock_oracle_address = Address::generate(&env);

    escrow_client.initialize(&asset.address, &mock_oracle_address);

    let event_expected = (
        contract_id.clone(),
        (INIT_TOPIC,).into_val(&env),
        (asset.address, mock_oracle_address).into_val(&env),
    );

    assert!(
        env.events().all().contains(event_expected),
        "Wrong event data emitted"
    );
}

#[test]
fn test_init() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EscrowContract);
    let escrow_client = EscrowContractClient::new(&env, &contract_id);

    let native_assset_address = native_asset_contract_address(&env);
    let mock_oracle_address = Address::generate(&env);

    escrow_client.initialize(&native_assset_address, &mock_oracle_address);

    let event_expected = (
        contract_id.clone(),
        (INIT_TOPIC,).into_val(&env),
        (native_assset_address, mock_oracle_address).into_val(&env),
    );

    assert!(
        env.events().all().contains(event_expected),
        "Wrong event data emitted"
    );
}
