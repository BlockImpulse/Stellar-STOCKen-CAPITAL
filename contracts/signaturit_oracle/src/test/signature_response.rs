#![cfg(test)]

extern crate std;
use soroban_sdk::{
    testutils::{Events, MockAuth, MockAuthInvoke},
    IntoVal, String,
};
use std::string::ToString;
use uuid::Uuid;

use crate::{
    events::OracleEvent,
    test::{oracle::OracleError, OracleTest, COMPLETED_TOPIC, FAILED_TOPIC},
};

#[test]
fn signature_response_call_fail() {
    let test = OracleTest::setup();

    let signaturit_id = String::from_str(&test.env, &Uuid::new_v4().to_string());

    // Register the process to the Oracle
    let oracle_id = test.escrow.oracle_register(&signaturit_id);

    let signature_response = false; // FAILED
    let document_hash: Option<String> = None;

    //  The Oracle should be trigger with the response
    test.oracle
        .mock_auths(&[MockAuth {
            address: &test.admin,
            invoke: &MockAuthInvoke {
                contract: &test.oracle.address,
                fn_name: "signature_response",
                args: (oracle_id, signature_response, document_hash.clone()).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .signature_response(&oracle_id, &signature_response, &document_hash);

    // Check SignatureResponse event
    let event_expected = (
        test.oracle.address.clone(),
        (
            OracleEvent::SignatureResponse(signaturit_id.clone(), oracle_id, signature_response)
                .name(),
        )
            .into_val(&test.env),
        (signaturit_id.clone(), oracle_id, signature_response).into_val(&test.env),
    );

    assert!(
        test.env.events().all().contains(event_expected),
        "SignatureResponse event not present"
    );

    // Check event on implementer
    let event_expected = (
        test.escrow.address,
        (FAILED_TOPIC,).into_val(&test.env),
        (signaturit_id).into_val(&test.env),
    );

    assert!(
        test.env.events().all().contains(event_expected),
        "Implementer event not present"
    );
}

#[test]
fn signature_response_call_success() {
    let test = OracleTest::setup();

    let signaturit_id = String::from_str(&test.env, &Uuid::new_v4().to_string());

    // Register the process to the Oracle
    let oracle_id = test.escrow.oracle_register(&signaturit_id);

    let signature_response = true; // Success
    let document_hash = Some(String::from_str(&test.env, "Test document hash"));

    //  The Oracle should be trigger with the response
    test.oracle
        .mock_auths(&[MockAuth {
            address: &test.admin,
            invoke: &MockAuthInvoke {
                contract: &test.oracle.address,
                fn_name: "signature_response",
                args: (oracle_id, signature_response, Some(document_hash.clone()))
                    .into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .signature_response(&oracle_id, &signature_response, &document_hash);

    // Check SignatureResponse event
    let event_expected = (
        test.oracle.address.clone(),
        (
            OracleEvent::SignatureResponse(signaturit_id.clone(), oracle_id, signature_response)
                .name(),
        )
            .into_val(&test.env),
        (signaturit_id.clone(), oracle_id, signature_response).into_val(&test.env),
    );

    assert!(
        test.env.events().all().contains(event_expected),
        "SignatureResponse event not present"
    );

    // Check event on implementer
    let event_expected = (
        test.escrow.address,
        (COMPLETED_TOPIC,).into_val(&test.env),
        (signaturit_id, document_hash.unwrap()).into_val(&test.env),
    );

    assert!(
        test.env.events().all().contains(event_expected),
        "Implementer event not present"
    );
}

#[test]
fn signature_response_only_admin() {
    let test = OracleTest::setup();

    let signaturit_id = String::from_str(&test.env, &Uuid::new_v4().to_string());

    // Register the process to the Oracle
    let oracle_id = test.escrow.oracle_register(&signaturit_id);

    let signature_response = true; // Success
    let document_hash = Some(String::from_str(&test.env, "Test document hash"));

    //  The Oracle should be trigger with the response
    let res = test
        .oracle
        .mock_auths(&[MockAuth {
            address: &test.alice,
            invoke: &MockAuthInvoke {
                contract: &test.oracle.address,
                fn_name: "signature_response",
                args: (oracle_id, signature_response, Some(document_hash.clone()))
                    .into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .try_signature_response(&oracle_id, &signature_response, &document_hash);

    assert!(res.is_err(), "callin with non-admin not reverted");
}

#[test]
fn signature_response_inexisting_id() {
    let test = OracleTest::setup();

    // An oracle id
    let oracle_id = 90;

    let signature_response = false; // FAILED
    let document_hash: Option<String> = None;

    //  The Oracle should be trigger with the response
    let res = test
        .oracle
        .mock_auths(&[MockAuth {
            address: &test.admin,
            invoke: &MockAuthInvoke {
                contract: &test.oracle.address,
                fn_name: "signature_response",
                args: (oracle_id, signature_response, document_hash.clone()).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .try_signature_response(&oracle_id, &signature_response, &document_hash);

    assert_eq!(res, Err(Ok(OracleError::ProcessNotFound.into())));
}

#[test]
fn signature_response_missing_doc_hash() {
    let test = OracleTest::setup();

    let signaturit_id = String::from_str(&test.env, &Uuid::new_v4().to_string());

    // Register the process to the Oracle
    let oracle_id = test.escrow.oracle_register(&signaturit_id);

    let signature_response = true; // Success
    let document_hash: Option<String> = None;

    //  The Oracle should be trigger with the response
    let res = test
        .oracle
        .mock_auths(&[MockAuth {
            address: &test.admin,
            invoke: &MockAuthInvoke {
                contract: &test.oracle.address,
                fn_name: "signature_response",
                args: (oracle_id, signature_response, Some(document_hash.clone()))
                    .into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .try_signature_response(&oracle_id, &signature_response, &document_hash);

    assert_eq!(res, Err(Ok(OracleError::MissingDocHash.into())));
}
