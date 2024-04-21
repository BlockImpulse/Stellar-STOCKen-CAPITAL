#![cfg(test)]

use crate::{
    events::INITIALIZED_TOPIC,
    test::{notes_nft::Error, NotesNFTTest},
};
use soroban_sdk::{testutils::Events, IntoVal, String};

#[test]
fn initialization() {
    let test = NotesNFTTest::setup_non_init();

    let name = String::from_str(&test.env, "Test NFT Name");
    let symbol = String::from_str(&test.env, "TEST_SYMBOL");

    test.notes_nft
        .initialize(&test.admin_escrow, &name, &symbol);

    let event_expected = (
        test.notes_nft.address.clone(),
        (INITIALIZED_TOPIC,).into_val(&test.env),
        (test.admin_escrow.clone(), name.clone(), symbol.clone()).into_val(&test.env),
    );

    assert!(
        test.env.events().all().contains(event_expected),
        "initialized event not present"
    );

    assert_eq!(test.notes_nft.admin(), test.admin_escrow, "wrong NFT admin");
    assert_eq!(test.notes_nft.total_supply(), 0, "wrong initial supply");

    assert_eq!(test.notes_nft.name(), name, "wrong name");
    assert_eq!(test.notes_nft.symbol(), symbol, "wrong symbol");
}

#[test]
fn double_initialization() {
    let test = NotesNFTTest::setup_non_init();

    let name = String::from_str(&test.env, "Test NFT Name");
    let symbol = String::from_str(&test.env, "TEST_SYMBOL");

    // First initialization
    test.notes_nft
        .initialize(&test.admin_escrow, &name, &symbol);

    let res = test
        .notes_nft
        .try_initialize(&test.admin_escrow, &name, &symbol);

    assert_eq!(res, Err(Ok(Error::AlreadyInit.into())));
}
