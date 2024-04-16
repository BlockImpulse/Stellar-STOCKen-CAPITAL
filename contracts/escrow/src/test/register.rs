#![cfg(test)]

extern crate std;
use std::string::ToString;

use crate::{
    events::REGISTER_TOPIC,
    test::utils::{create_token_contract, STOCKEN_ID_1},
    types::{SignatureStatus, SignatureTxEscrow},
    EscrowContract, EscrowContractClient,
};

use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, Events},
    Address, Env, IntoVal, String, Val, Vec,
};

use uuid::Uuid;

#[test]
fn test_new_register() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, EscrowContract);
    let escrow_client = EscrowContractClient::new(&env, &contract_id);

    // Init the escrow
    let admin_address = Address::generate(&env);
    let (asset, asset_admin) = create_token_contract(&env, &admin_address);
    let mock_oracle_address = Address::generate(&env);
    escrow_client.initialize(&asset.address, &mock_oracle_address);

    // Add a proposal
    let stocken_id = String::from_str(&env, STOCKEN_ID_1);
    let proposer_address = Address::generate(&env);
    let amount_asked: i128 = 900000;
    escrow_client.add_proposal(&stocken_id, &proposer_address, &amount_asked);

    // Pick a escrow
    // The sginaturit ID is an UUID
    let signaturit_id = String::from_str(&env, &Uuid::new_v4().to_string());
    let picker_address = Address::generate(&env);
    let amount_to_give: i128 = 1800000;

    // Provide tokens to the picker address
    asset_admin.mint(&picker_address, &amount_to_give);

    escrow_client.register_escrow(
        &stocken_id,
        &signaturit_id,
        &picker_address,
        &amount_to_give,
    );

    let expected_signature_tx = SignatureTxEscrow {
        id: signaturit_id,
        propose_id: stocken_id,
        oracle_id: 0,
        buyer: picker_address,
        receiver: proposer_address,
        funds: amount_to_give,
        status: SignatureStatus::Progress,
    };

    let event_expected: (Address, Vec<Val>, Val) = (
        contract_id.clone(),
        (REGISTER_TOPIC, symbol_short!("New")).into_val(&env),
        (expected_signature_tx).into_val(&env),
    );

    assert!(
        env.events().all().contains(event_expected),
        "Wrong event data emitted"
    );
}
