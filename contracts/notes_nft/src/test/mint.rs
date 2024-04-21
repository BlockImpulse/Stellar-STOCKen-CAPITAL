#![cfg(test)]

use crate::{
    events::Event,
    test::{Error, NotesNFTTest},
};
use soroban_sdk::{
    testutils::{Events, MockAuth, MockAuthInvoke},
    IntoVal, String,
};

#[test]
fn mint() {
    let test = NotesNFTTest::setup();

    assert_eq!(test.notes_nft.total_supply(), 0, "wrong total supply");

    // Mint one NFT to Alice address with a doc hash
    assert_eq!(
        test.notes_nft.balance_of(&test.alice),
        0,
        "wrong alice balance"
    );

    // Document hash
    let doc_hash_1 = String::from_bytes(&test.env, &[0; 32]);

    test.notes_nft
        .mock_auths(&[MockAuth {
            address: &test.admin_escrow.clone(),
            invoke: &MockAuthInvoke {
                contract: &test.notes_nft.address,
                fn_name: "mint",
                args: (test.alice.clone(), doc_hash_1.clone()).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .mint(&test.alice, &doc_hash_1);

    // Check the new state
    let expected_token_id_1: u32 = 0;

    assert_eq!(test.notes_nft.owner_of(&expected_token_id_1), test.alice);
    assert_eq!(test.notes_nft.token_uri(&expected_token_id_1), doc_hash_1);
    assert_eq!(test.notes_nft.total_supply(), 1, "wrong total supply");
    assert_eq!(
        test.notes_nft.balance_of(&test.alice),
        1,
        "wrong alice balance"
    );

    // Check the values on the event
    let event_expected_1 = (
        test.notes_nft.address.clone(),
        (Event::Mint.name(),).into_val(&test.env),
        (test.alice.clone(), expected_token_id_1).into_val(&test.env),
    );

    assert!(
        test.env.events().all().contains(event_expected_1),
        "mint event not present"
    );

    // Mint one NFT to Bob address with a doc hash
    assert_eq!(test.notes_nft.balance_of(&test.bob), 0, "wrong bob balance");

    // Document hash
    let doc_hash_2 = String::from_bytes(&test.env, &[1; 32]);

    // Mint one NFT to Alice address with a doc hash
    test.notes_nft
        .mock_auths(&[MockAuth {
            address: &test.admin_escrow.clone(),
            invoke: &MockAuthInvoke {
                contract: &test.notes_nft.address,
                fn_name: "mint",
                args: (test.bob.clone(), doc_hash_2.clone()).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .mint(&test.bob, &doc_hash_2);

    // Check the new state
    let expected_token_id_2: u32 = 1;
    assert_eq!(test.notes_nft.owner_of(&expected_token_id_2), test.bob);
    assert_eq!(test.notes_nft.token_uri(&expected_token_id_2), doc_hash_2);
    assert_eq!(test.notes_nft.total_supply(), 2, "wrong total supply");
    assert_eq!(
        test.notes_nft.balance_of(&test.alice),
        1,
        "wrong bob balance"
    );

    // Check the values on the vent
    let event_expected_2 = (
        test.notes_nft.address.clone(),
        (Event::Mint.name(),).into_val(&test.env),
        (test.bob.clone(), expected_token_id_2).into_val(&test.env),
    );

    assert!(
        test.env.events().all().contains(event_expected_2),
        "mint event not present"
    );
}

#[test]
fn no_owner() {
    let test = NotesNFTTest::setup();

    let res = test.notes_nft.try_owner_of(&0);
    assert_eq!(res, Err(Ok(Error::NotNFT.into())));

    let res_1 = test.notes_nft.try_token_uri(&0);
    assert_eq!(res_1, Err(Ok(Error::NotNFT.into())));
}

#[test]
fn mint_not_admin() {
    let test = NotesNFTTest::setup();

    assert_eq!(test.notes_nft.total_supply(), 0);
    assert_eq!(test.notes_nft.balance_of(&test.alice), 0);

    let doc_hash = String::from_bytes(&test.env, &[0; 32]);

    let res = test
        .notes_nft
        .mock_auths(&[MockAuth {
            address: &test.alice.clone(),
            invoke: &MockAuthInvoke {
                contract: &test.notes_nft.address,
                fn_name: "mint",
                args: (test.alice.clone(), doc_hash.clone()).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .try_mint(&test.alice, &doc_hash);

    assert!(res.is_err(), "It is not an error");
    assert_eq!(
        test.notes_nft.total_supply(),
        0,
        "total supply was updated with non admin"
    );
    assert_eq!(
        test.notes_nft.balance_of(&test.alice),
        0,
        "balance was updated with non admin"
    );
}
