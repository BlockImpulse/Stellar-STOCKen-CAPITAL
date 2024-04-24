#![cfg(test)]

extern crate std;
use soroban_sdk::{testutils::Events, IntoVal, String};
use std::string::ToString;
use uuid::Uuid;

use crate::{
    events::OracleEvent,
    test::{oracle::OracleError, OracleTest},
};

#[test]
fn register_new_process() {
    let test = OracleTest::setup();

    let signaturit_id = String::from_str(&test.env, &Uuid::new_v4().to_string());

    let expected_id: u32 = 0;

    let oracle_id = test
        .oracle
        .register_new_signature_process(&test.alice, &signaturit_id);

    assert_eq!(oracle_id, expected_id);

    // Check NewSignatureProcess event
    let event_expected = (
        test.oracle.address.clone(),
        (OracleEvent::NewSignatureProcess(signaturit_id.clone(), expected_id.clone()).name(),)
            .into_val(&test.env),
        (signaturit_id, expected_id).into_val(&test.env),
    );

    assert!(
        test.env.events().all().contains(event_expected),
        "NewSignatureProcess event not present"
    );
}

#[test]
fn multiple_register_new_process() {
    let test = OracleTest::setup();

    let mut expected_id: u32 = 0;

    // First registry
    let signaturit_id_1 = String::from_str(&test.env, &Uuid::new_v4().to_string());

    let oracle_id_1 = test
        .oracle
        .register_new_signature_process(&test.alice, &signaturit_id_1);

    assert_eq!(oracle_id_1, expected_id);

    // Check NewSignatureProcess event
    let event_expected_1 = (
        test.oracle.address.clone(),
        (OracleEvent::NewSignatureProcess(signaturit_id_1.clone(), expected_id.clone()).name(),)
            .into_val(&test.env),
        (signaturit_id_1, expected_id).into_val(&test.env),
    );

    assert!(
        test.env.events().all().contains(event_expected_1),
        "NewSignatureProcess event not present"
    );

    // Second registry
    expected_id += 1;
    let signaturit_id_2 = String::from_str(&test.env, &Uuid::new_v4().to_string());

    let oracle_id_2 = test
        .oracle
        .register_new_signature_process(&test.bob, &signaturit_id_2);

    assert_eq!(oracle_id_2, expected_id);

    // Check NewSignatureProcess event
    let event_expected_2 = (
        test.oracle.address.clone(),
        (OracleEvent::NewSignatureProcess(signaturit_id_2.clone(), expected_id.clone()).name(),)
            .into_val(&test.env),
        (signaturit_id_2, expected_id).into_val(&test.env),
    );

    assert!(
        test.env.events().all().contains(event_expected_2),
        "NewSignatureProcess event not present"
    );

    // Third registry
    expected_id += 1;
    let signaturit_id_3 = String::from_str(&test.env, &Uuid::new_v4().to_string());

    let oracle_id_3 = test
        .oracle
        .register_new_signature_process(&test.alice, &signaturit_id_3);

    assert_eq!(oracle_id_3, expected_id);

    // Check NewSignatureProcess event
    let event_expected_3 = (
        test.oracle.address.clone(),
        (OracleEvent::NewSignatureProcess(signaturit_id_3.clone(), expected_id.clone()).name(),)
            .into_val(&test.env),
        (signaturit_id_3, expected_id).into_val(&test.env),
    );

    assert!(
        test.env.events().all().contains(event_expected_3),
        "NewSignatureProcess event not present"
    );
}

#[test]
fn register_new_process_already_exist() {
    let test = OracleTest::setup();

    let signaturit_id = String::from_str(&test.env, &Uuid::new_v4().to_string());

    _ = test
        .oracle
        .register_new_signature_process(&test.alice, &signaturit_id);

    let res_same_caller = test
        .oracle
        .try_register_new_signature_process(&test.alice, &signaturit_id);

    let res_diff_caller = test
        .oracle
        .try_register_new_signature_process(&test.bob, &signaturit_id);

    assert_eq!(
        res_same_caller,
        Err(Ok(OracleError::SignatureIdAlredyExist.into()))
    );
    assert_eq!(
        res_diff_caller,
        Err(Ok(OracleError::SignatureIdAlredyExist.into()))
    );
}