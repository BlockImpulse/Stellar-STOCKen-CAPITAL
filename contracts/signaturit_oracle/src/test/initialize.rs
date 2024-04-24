#![cfg(test)]

use crate::{
    events::OracleEvent,
    test::{oracle::OracleError, OracleTest},
};
use soroban_sdk::{testutils::Events, IntoVal};

#[test]
fn before_initialization() {
    let test = OracleTest::setup_non_init();

    let res_get_admin = test.oracle.try_get_admin();

    assert_eq!(res_get_admin, Err(Ok(OracleError::NotInit.into())));
}

#[test]
fn initialization() {
    let test = OracleTest::setup_non_init();

    test.oracle.initialize(&test.admin);

    // Check Initialized event
    let event_expected = (
        test.oracle.address.clone(),
        (OracleEvent::Initialized(test.admin.clone()).name(),).into_val(&test.env),
        (&test.admin,).into_val(&test.env),
    );

    assert!(
        test.env.events().all().contains(event_expected),
        "initialized event not present"
    );

    assert_eq!(test.oracle.get_admin(), test.admin);
}

#[test]
fn double_initialization() {
    let test = OracleTest::setup_non_init();

    // First initialization
    test.oracle.initialize(&test.admin);

    let res = test.oracle.try_initialize(&test.admin);

    assert_eq!(res, Err(Ok(OracleError::AlreadyInit.into())));
}
