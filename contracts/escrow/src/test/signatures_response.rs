#![cfg(test)]

extern crate std;
use core::ops::Mul;
use std::string::ToString;

use crate::{
    events::EscrowEvent,
    test::{
        escrow::{NullableString, ProposalStatus, SignatureStatus},
        EscrowTest, STOCKEN_ID_1,
    },
};
use soroban_sdk::{
    testutils::{Events, MockAuth, MockAuthInvoke},
    IntoVal, String,
};
use uuid::Uuid;

#[test]
fn failed_signature() {
    let test = EscrowTest::setup();

    // Add a proposal
    let stocken_id = String::from_str(&test.env, STOCKEN_ID_1);
    let amount_asked: i128 = 10_000_000_000_000_000_000; // 10 tokens
    test.escrow
        .add_proposal(&stocken_id, &test.alice, &amount_asked);

    // Pick a escrow
    // The sginaturit ID is an UUID
    let signaturit_id = String::from_str(&test.env, &Uuid::new_v4().to_string());
    let amount_to_give: i128 = amount_asked.mul(2);

    let balance_before_alice = test.token.balance(&test.alice);
    let balance_before_bob = test.token.balance(&test.bob);

    test.escrow
        .mock_auths(&[MockAuth {
            address: &test.bob,
            invoke: &MockAuthInvoke {
                contract: &test.token.address,
                fn_name: "transfer",
                args: (
                    test.bob.clone(),
                    test.escrow.address.clone(),
                    amount_to_give.clone(),
                )
                    .into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .register_escrow(&stocken_id, &signaturit_id, &test.bob, &amount_to_give);

    // The proposal owner should not receive the funds until the signature
    // process is completed
    assert_eq!(
        test.token.balance(&test.alice),
        balance_before_alice,
        "proposal owner balance has changed"
    );

    assert_eq!(
        test.token.balance(&test.bob),
        balance_before_bob - amount_to_give,
        "funder balance is not correct"
    );

    assert_eq!(
        test.token.balance(&test.escrow.address),
        amount_to_give,
        "escrow contract does not have the correct balance"
    );

    // Escrow wait for the Oracle callback
    // This is the oracle ID assigned to the register (it is on the event)
    let oracle_id: u32 = 0;
    let signature_response = false; // FAILED
    let document_hash: Option<String> = None;

    let before_response_alice_balance = test.token.balance(&test.alice);
    let before_response_bob_balance = test.token.balance(&test.bob);
    let before_response_escrow_balance = test.token.balance(&test.escrow.address);

    // Trigger the Oracle to do the callback
    // The admin of the Oracle is the only address that can trigger a fn
    test.oracle
        .mock_auths(&[MockAuth {
            address: &test.admin, // Oracle admin
            invoke: &MockAuthInvoke {
                contract: &test.oracle.address,
                fn_name: "signature_response",
                args: (oracle_id, signature_response, document_hash.clone()).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .signature_response(&oracle_id, &signature_response, &document_hash);

    // Check SignedFailed event
    let event_expected = (
        test.escrow.address.clone(),
        (
            EscrowEvent::SignedFailed(signaturit_id.clone(), stocken_id.clone(), test.bob.clone())
                .name(),
        )
            .into_val(&test.env),
        (signaturit_id.clone(), stocken_id.clone(), test.bob.clone()).into_val(&test.env),
    );

    assert!(
        test.env.events().all().contains(event_expected),
        "SignedFailed event not present"
    );

    // Check the final status
    let proposal = test.escrow.get_proposal(&stocken_id);
    let signature_process = test.escrow.get_signature_tx_escrow(&signaturit_id);

    assert_eq!(
        proposal.status,
        ProposalStatus::Actived,
        "proposal status was not updated"
    );
    assert_eq!(proposal.signature_tx_linked, NullableString::None);

    assert_eq!(
        signature_process.status,
        SignatureStatus::Canceled,
        "signature status was not updated"
    );

    assert_eq!(
        test.token.balance(&test.alice),
        before_response_alice_balance
    );
    assert_eq!(
        test.token.balance(&test.bob),
        before_response_bob_balance + amount_to_give
    );
    assert_eq!(
        test.token.balance(&test.escrow.address),
        before_response_escrow_balance - amount_to_give
    );
}

#[test]
fn success_signature() {
    let test = EscrowTest::setup();

    // Add a proposal
    let stocken_id = String::from_str(&test.env, STOCKEN_ID_1);
    let amount_asked: i128 = 10_000_000_000_000_000_000; // 10 tokens
    test.escrow
        .add_proposal(&stocken_id, &test.alice, &amount_asked);

    // Pick a escrow
    // The sginaturit ID is an UUID
    let signaturit_id = String::from_str(&test.env, &Uuid::new_v4().to_string());
    let amount_to_give: i128 = amount_asked.mul(2);

    let balance_before_alice = test.token.balance(&test.alice);
    let balance_before_bob = test.token.balance(&test.bob);

    test.escrow
        .mock_auths(&[MockAuth {
            address: &test.bob,
            invoke: &MockAuthInvoke {
                contract: &test.token.address,
                fn_name: "transfer",
                args: (
                    test.bob.clone(),
                    test.escrow.address.clone(),
                    amount_to_give.clone(),
                )
                    .into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .register_escrow(&stocken_id, &signaturit_id, &test.bob, &amount_to_give);

    // The proposal owner should not receive the funds until the signature
    // process is completed
    assert_eq!(
        test.token.balance(&test.alice),
        balance_before_alice,
        "proposal owner balance has changed"
    );

    assert_eq!(
        test.token.balance(&test.bob),
        balance_before_bob - amount_to_give,
        "funder balance is not correct"
    );

    assert_eq!(
        test.token.balance(&test.escrow.address),
        amount_to_give,
        "escrow contract does not have the correct balance"
    );

    // Escrow wait for the Oracle callback
    // This is the oracle ID assigned to the register (it is on the event)
    let oracle_id: u32 = 0;
    let signature_response = true; // FAILED
    let document_hash = Some(String::from_str(&test.env, "Test document hash"));
    let token_nft_id: u32 = 0;

    let before_response_alice_balance = test.token.balance(&test.alice);
    let before_response_bob_balance = test.token.balance(&test.bob);
    let before_response_escrow_balance = test.token.balance(&test.escrow.address);

    // Trigger the Oracle to do the callback
    // The admin of the Oracle is the only address that can trigger a fn
    test.oracle
        .mock_auths(&[MockAuth {
            address: &test.admin, // Oracle admin
            invoke: &MockAuthInvoke {
                contract: &test.oracle.address,
                fn_name: "signature_response",
                args: (oracle_id, signature_response, document_hash.clone()).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .signature_response(&oracle_id, &signature_response, &document_hash);

    // Check SignedCompleted event
    let event_expected = (
        test.escrow.address.clone(),
        (EscrowEvent::SignedCompleted(
            signaturit_id.clone(),
            stocken_id.clone(),
            test.bob.clone(),
            test.alice.clone(),
            amount_to_give.clone(),
            token_nft_id.clone(),
        )
        .name(),)
            .into_val(&test.env),
        (
            signaturit_id.clone(),
            stocken_id.clone(),
            test.bob.clone(),
            test.alice.clone(),
            amount_to_give.clone(),
            token_nft_id.clone(),
        )
            .into_val(&test.env),
    );

    assert!(
        test.env.events().all().contains(event_expected),
        "SignedCompleted event not present"
    );

    // Check the final status
    let proposal = test.escrow.get_proposal(&stocken_id);
    let signature_process = test.escrow.get_signature_tx_escrow(&signaturit_id);

    assert_eq!(
        proposal.status,
        ProposalStatus::Completed,
        "proposal status was not updated"
    );
    assert_eq!(
        proposal.signature_tx_linked,
        NullableString::Some(signature_process.id.clone())
    );

    assert_eq!(
        signature_process.status,
        SignatureStatus::Completed,
        "signature status was not updated"
    );

    assert_eq!(
        test.token.balance(&test.alice),
        before_response_alice_balance + amount_to_give
    );
    assert_eq!(test.token.balance(&test.bob), before_response_bob_balance);
    assert_eq!(
        test.token.balance(&test.escrow.address),
        before_response_escrow_balance - amount_to_give
    );
}
