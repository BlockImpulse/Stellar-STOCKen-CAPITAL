#![cfg(test)]

use crate::{
    events::Event,
    test::{notes_nft::Error, NotesNFTTest},
};
use soroban_sdk::{
    testutils::{Events, MockAuth, MockAuthInvoke},
    Address, IntoVal, String,
};

#[test]
fn transfer() {
    let test = NotesNFTTest::setup();

    // Mint one NFT to Alice address with a doc hash
    assert_eq!(
        test.notes_nft.balance_of(&test.alice),
        0,
        "wrong alice balance"
    );
    assert_eq!(test.notes_nft.balance_of(&test.bob), 0, "wrong bob balance");

    // Document hash
    let doc_hash_1 = String::from_bytes(&test.env, &[0; 32]);
    let expected_nft_id: u32 = 0;

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

    assert_eq!(
        test.notes_nft.balance_of(&test.alice),
        1,
        "wrong alice balance"
    );
    assert_eq!(
        test.notes_nft.owner_of(&expected_nft_id),
        test.alice,
        "wrong owner"
    );

    // Transfer from Alice to Bob
    test.notes_nft
        .mock_auths(&[MockAuth {
            address: &test.alice.clone(),
            invoke: &MockAuthInvoke {
                contract: &test.notes_nft.address,
                fn_name: "transfer_from",
                args: (
                    test.alice.clone(),
                    test.alice.clone(),
                    test.bob.clone(),
                    expected_nft_id.clone(),
                )
                    .into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .transfer_from(&test.alice, &test.alice, &test.bob, &expected_nft_id);

    assert_eq!(
        test.notes_nft.balance_of(&test.alice),
        0,
        "wrong alice balance"
    );
    assert_eq!(test.notes_nft.balance_of(&test.bob), 1, "wrong bob balance");
    assert_eq!(
        test.notes_nft.owner_of(&expected_nft_id),
        test.bob,
        "wrong owner"
    );

    // Check the values on the event
    let event_expected = (
        test.notes_nft.address.clone(),
        (Event::Transfer.name(),).into_val(&test.env),
        (
            test.alice.clone(),
            test.bob.clone(),
            expected_nft_id.clone(),
        )
            .into_val(&test.env),
    );

    assert!(
        test.env.events().all().contains(event_expected),
        "transfer event not present"
    );
}

#[test]
fn approved() {
    let test = NotesNFTTest::setup();

    // Mint one NFT to Alice address with a doc hash
    assert_eq!(
        test.notes_nft.balance_of(&test.alice),
        0,
        "wrong alice balance"
    );
    assert_eq!(test.notes_nft.balance_of(&test.bob), 0, "wrong bob balance");

    // Document hash
    let doc_hash_1 = String::from_bytes(&test.env, &[0; 32]);
    let expected_nft_id: u32 = 0;
    let ttl = 1000;

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

    assert_eq!(
        test.notes_nft.balance_of(&test.alice),
        1,
        "wrong alice balance"
    );
    assert_eq!(
        test.notes_nft.owner_of(&expected_nft_id),
        test.alice,
        "wrong owner"
    );

    // Approve Bob to manage the Alice token
    test.notes_nft
        .mock_auths(&[MockAuth {
            address: &test.alice.clone(),
            invoke: &MockAuthInvoke {
                contract: &test.notes_nft.address,
                fn_name: "approve",
                args: (
                    test.alice.clone(),
                    Some(test.bob.clone()),
                    expected_nft_id,
                    ttl,
                )
                    .into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .approve(&test.alice, &Some(test.bob.clone()), &expected_nft_id, &ttl);

    let approved_address = test.notes_nft.get_approved(&expected_nft_id);

    assert!(approved_address.is_some());
    assert_eq!(approved_address.unwrap(), test.bob);
    assert_eq!(test.notes_nft.balance_of(&test.alice), 1);
    assert_eq!(test.notes_nft.balance_of(&test.bob), 0);

    // Check the values on the event
    let event_expected = (
        test.notes_nft.address.clone(),
        (Event::Approve.name(),).into_val(&test.env),
        (
            test.alice.clone(),
            test.bob.clone(),
            expected_nft_id.clone(),
        )
            .into_val(&test.env),
    );

    assert!(
        test.env.events().all().contains(event_expected),
        "approve event not present"
    );

    // Move the token from Alice using Bob
    test.notes_nft
        .mock_auths(&[MockAuth {
            address: &test.bob.clone(),
            invoke: &MockAuthInvoke {
                contract: &test.notes_nft.address,
                fn_name: "transfer_from",
                args: (
                    test.bob.clone(),
                    test.alice.clone(),
                    test.bob.clone(),
                    expected_nft_id.clone(),
                )
                    .into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .transfer_from(&test.bob, &test.alice, &test.bob, &expected_nft_id);

    assert_eq!(
        test.notes_nft.balance_of(&test.alice),
        0,
        "wrong alice balance"
    );
    assert_eq!(test.notes_nft.balance_of(&test.bob), 1, "wrong bob balance");
    assert_eq!(
        test.notes_nft.owner_of(&expected_nft_id),
        test.bob,
        "wrong owner"
    );
}

#[test]
fn approved_for_all() {
    let test = NotesNFTTest::setup();

    // Mint one NFT to Alice address with a doc hash
    assert_eq!(
        test.notes_nft.balance_of(&test.alice),
        0,
        "wrong alice balance"
    );
    assert_eq!(test.notes_nft.balance_of(&test.bob), 0, "wrong bob balance");

    // Document hash
    let doc_hash_1 = String::from_bytes(&test.env, &[0; 32]);
    let expected_nft_id: u32 = 0;
    let is_approved = true;
    let ttl = 1000;

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

    assert_eq!(
        test.notes_nft.balance_of(&test.alice),
        1,
        "wrong alice balance"
    );
    assert_eq!(
        test.notes_nft.owner_of(&expected_nft_id),
        test.alice,
        "wrong owner"
    );

    test.notes_nft
        .mock_auths(&[MockAuth {
            address: &test.alice.clone(),
            invoke: &MockAuthInvoke {
                contract: &test.notes_nft.address,
                fn_name: "set_approval_for_all",
                args: (
                    test.alice.clone(),
                    test.alice.clone(),
                    test.bob.clone(),
                    is_approved,
                    ttl,
                )
                    .into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .set_approval_for_all(&test.alice, &test.alice, &test.bob, &is_approved, &ttl);

    assert_eq!(
        test.notes_nft.is_approval_for_all(&test.alice, &test.bob),
        is_approved
    );

    // Check the values on the event
    let event_expected = (
        test.notes_nft.address.clone(),
        (Event::ApproveForAll.name(),).into_val(&test.env),
        (test.alice.clone(), test.bob.clone(), is_approved).into_val(&test.env),
    );

    assert!(
        test.env.events().all().contains(event_expected),
        "approve_for_all event not present"
    );

    // Move the token from Alice using Bob
    test.notes_nft
        .mock_auths(&[MockAuth {
            address: &test.bob.clone(),
            invoke: &MockAuthInvoke {
                contract: &test.notes_nft.address,
                fn_name: "transfer_from",
                args: (
                    test.bob.clone(),
                    test.alice.clone(),
                    test.bob.clone(),
                    expected_nft_id.clone(),
                )
                    .into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .transfer_from(&test.bob, &test.alice, &test.bob, &expected_nft_id);

    assert_eq!(
        test.notes_nft.balance_of(&test.alice),
        0,
        "wrong alice balance"
    );
    assert_eq!(test.notes_nft.balance_of(&test.bob), 1, "wrong bob balance");
    assert_eq!(
        test.notes_nft.owner_of(&expected_nft_id),
        test.bob,
        "wrong owner"
    );
}

#[test]
fn not_approved() {
    let test = NotesNFTTest::setup();

    // Mint one NFT to Alice address with a doc hash
    assert_eq!(
        test.notes_nft.balance_of(&test.alice),
        0,
        "wrong alice balance"
    );
    assert_eq!(test.notes_nft.balance_of(&test.bob), 0, "wrong bob balance");

    // Document hash
    let doc_hash_1 = String::from_bytes(&test.env, &[0; 32]);
    let expected_nft_id: u32 = 0;

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

    assert_eq!(
        test.notes_nft.balance_of(&test.alice),
        1,
        "wrong alice balance"
    );
    assert_eq!(
        test.notes_nft.owner_of(&expected_nft_id),
        test.alice,
        "wrong owner"
    );

    // Move the token from Alice using Bob
    let res = test
        .notes_nft
        .mock_auths(&[MockAuth {
            address: &test.bob.clone(),
            invoke: &MockAuthInvoke {
                contract: &test.notes_nft.address,
                fn_name: "transfer_from",
                args: (
                    test.bob.clone(),
                    test.alice.clone(),
                    test.bob.clone(),
                    expected_nft_id.clone(),
                )
                    .into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .try_transfer_from(&test.bob, &test.alice, &test.bob, &expected_nft_id);

    assert_eq!(res, Err(Ok(Error::NotAuthorized.into())));
}

#[test]
fn remove_approved() {
    let test = NotesNFTTest::setup();

    // Mint one NFT to Alice address with a doc hash
    assert_eq!(
        test.notes_nft.balance_of(&test.alice),
        0,
        "wrong alice balance"
    );
    assert_eq!(test.notes_nft.balance_of(&test.bob), 0, "wrong bob balance");

    // Document hash
    let doc_hash_1 = String::from_bytes(&test.env, &[0; 32]);
    let expected_nft_id: u32 = 0;
    let ttl = 1000;

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

    assert_eq!(
        test.notes_nft.balance_of(&test.alice),
        1,
        "wrong alice balance"
    );
    assert_eq!(
        test.notes_nft.owner_of(&expected_nft_id),
        test.alice,
        "wrong owner"
    );

    // Approve Bob to manage the Alice token
    test.notes_nft
        .mock_auths(&[MockAuth {
            address: &test.alice.clone(),
            invoke: &MockAuthInvoke {
                contract: &test.notes_nft.address,
                fn_name: "approve",
                args: (
                    test.alice.clone(),
                    Some(test.bob.clone()),
                    expected_nft_id,
                    ttl,
                )
                    .into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .approve(&test.alice, &Some(test.bob.clone()), &expected_nft_id, &ttl);

    let approved_address = test.notes_nft.get_approved(&expected_nft_id);

    assert!(approved_address.is_some());
    assert_eq!(approved_address.unwrap(), test.bob);

    // Remove Bob to manage the Alice token
    test.notes_nft
        .mock_auths(&[MockAuth {
            address: &test.alice.clone(),
            invoke: &MockAuthInvoke {
                contract: &test.notes_nft.address,
                fn_name: "approve",
                args: (test.alice.clone(), None::<Address>, expected_nft_id, ttl)
                    .into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .approve(&test.alice, &None, &expected_nft_id, &ttl);

    let approved_address = test.notes_nft.get_approved(&expected_nft_id);

    assert!(approved_address.is_none());

    // Move the token from Alice using Bob
    let res = test
        .notes_nft
        .mock_auths(&[MockAuth {
            address: &test.bob.clone(),
            invoke: &MockAuthInvoke {
                contract: &test.notes_nft.address,
                fn_name: "transfer_from",
                args: (
                    test.bob.clone(),
                    test.alice.clone(),
                    test.bob.clone(),
                    expected_nft_id.clone(),
                )
                    .into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .try_transfer_from(&test.bob, &test.alice, &test.bob, &expected_nft_id);

    assert_eq!(res, Err(Ok(Error::NotAuthorized.into())));
}

#[test]
fn remove_approved_for_all() {
    let test = NotesNFTTest::setup();

    // Mint one NFT to Alice address with a doc hash
    assert_eq!(
        test.notes_nft.balance_of(&test.alice),
        0,
        "wrong alice balance"
    );
    assert_eq!(test.notes_nft.balance_of(&test.bob), 0, "wrong bob balance");

    // Document hash
    let doc_hash_1 = String::from_bytes(&test.env, &[0; 32]);
    let expected_nft_id: u32 = 0;
    let is_approved = true;
    let ttl = 1000;

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

    assert_eq!(
        test.notes_nft.balance_of(&test.alice),
        1,
        "wrong alice balance"
    );
    assert_eq!(
        test.notes_nft.owner_of(&expected_nft_id),
        test.alice,
        "wrong owner"
    );

    test.notes_nft
        .mock_auths(&[MockAuth {
            address: &test.alice.clone(),
            invoke: &MockAuthInvoke {
                contract: &test.notes_nft.address,
                fn_name: "set_approval_for_all",
                args: (
                    test.alice.clone(),
                    test.alice.clone(),
                    test.bob.clone(),
                    is_approved,
                    ttl,
                )
                    .into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .set_approval_for_all(&test.alice, &test.alice, &test.bob, &is_approved, &ttl);

    assert_eq!(
        test.notes_nft.is_approval_for_all(&test.alice, &test.bob),
        is_approved
    );

    // Remove the approve for all
    let is_approved = false;

    test.notes_nft
        .mock_auths(&[MockAuth {
            address: &test.alice.clone(),
            invoke: &MockAuthInvoke {
                contract: &test.notes_nft.address,
                fn_name: "set_approval_for_all",
                args: (
                    test.alice.clone(),
                    test.alice.clone(),
                    test.bob.clone(),
                    is_approved,
                    ttl,
                )
                    .into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .set_approval_for_all(&test.alice, &test.alice, &test.bob, &is_approved, &ttl);
    ///////////////////////////

    // Check the values on the event
    let event_expected = (
        test.notes_nft.address.clone(),
        (Event::ApproveForAll.name(),).into_val(&test.env),
        (test.alice.clone(), test.bob.clone(), is_approved).into_val(&test.env),
    );

    assert!(
        test.env.events().all().contains(event_expected),
        "approve_for_all event not present"
    );

    // Move the token from Alice using Bob
    let res = test
        .notes_nft
        .mock_auths(&[MockAuth {
            address: &test.bob.clone(),
            invoke: &MockAuthInvoke {
                contract: &test.notes_nft.address,
                fn_name: "transfer_from",
                args: (
                    test.bob.clone(),
                    test.alice.clone(),
                    test.bob.clone(),
                    expected_nft_id.clone(),
                )
                    .into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .try_transfer_from(&test.bob, &test.alice, &test.bob, &expected_nft_id);

    assert_eq!(res, Err(Ok(Error::NotAuthorized.into())));
}
