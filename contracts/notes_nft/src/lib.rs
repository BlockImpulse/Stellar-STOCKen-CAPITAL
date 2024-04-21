#![no_std]

pub mod erc721traits;
mod errors;
mod events;
mod storage;
mod test;
mod types;

pub use crate::erc721traits::enumerable::ERC721Enumerable;
pub use crate::erc721traits::erc721::ERC721;
pub use crate::erc721traits::metadata::ERC721Metadata;
pub use errors::*;
pub use events::*;
pub use storage::Storage;
pub use types::*;

use soroban_sdk::{
    contract, contractimpl, panic_with_error, Address, Env, IntoVal, Map, String, Val, Vec,
};

fn get_admin(env: &Env) -> Address {
    Admin::Escrow.get(env).unwrap()
}

#[contract]
pub struct NotesNFTContract;

#[contractimpl]
impl NotesNFTContract {
    pub fn initialize(env: Env, escrow: Address, name: String, symbol: String) {
        if Admin::Escrow.has(&env) {
            panic_with_error!(env, Error::AlreadyInit);
        }
        Admin::Escrow.set(&env, &escrow);

        env.storage().instance().extend_ttl(10000, 10000);

        DatakeyMetadata::Name.set(&env, &name);
        DatakeyMetadata::Symbol.set(&env, &symbol);

        DataKeyEnumerable::CounterId.set(&env, &u32::MIN);
        DataKeyEnumerable::OwnedTokenIndices.set(&env, &Vec::<u32>::new(&env));
        DataKeyEnumerable::TokenIdToIndex.set(&env, &Map::<u32, u32>::new(&env));

        env.events()
            .publish((INITIALIZED_TOPIC,), (escrow, name, symbol));
    }

    pub fn admin(env: Env) -> Address {
        get_admin(&env)
    }

    pub fn owner_of(env: Env, token_id: u32) -> Address {
        if let Some(addr) = DataKey::TokenOwner(token_id).get::<Address>(&env) {
            addr
        } else {
            panic_with_error!(&env, Error::NotNFT);
        }
    }

    pub fn mint(env: Env, to: Address, note_document_hash: String) {
        get_admin(&env).require_auth();

        let new_token_id = DataKeyEnumerable::CounterId.get::<u32>(&env).unwrap();

        Self::internal_mint(&env, to, new_token_id, note_document_hash);

        DataKeyEnumerable::CounterId.set(&env, &(new_token_id + 1u32))
    }

    fn internal_mint(env: &Env, to: Address, token_id: u32, uri: String) {
        if !DataKey::TokenOwner(token_id).has(env) {
            DataKey::TokenOwner(token_id).set(env, &to);
            DatakeyMetadata::Uri(token_id).set(env, &uri);

            // A vector containing indices of tokens owned.
            let mut owned_token_indices: Vec<u32> =
                DataKeyEnumerable::OwnedTokenIndices.get(env).unwrap();

            // A map linking token IDs to their indices
            let mut token_id_to_index_map: Map<u32, u32> =
                DataKeyEnumerable::TokenIdToIndex.get(env).unwrap();

            // Related to an especific owner:
            // A vector containing ids of tokens owned by a specific address:
            let mut owner_token_indices: Vec<u32> =
                DataKeyEnumerable::OwnerOwnedTokenIds(to.clone())
                    .get(env)
                    .unwrap_or_else(|| Vec::new(env));

            // A map linking token IDs to their indices for a specific address.
            let mut owner_token_index: Map<u32, u32> =
                DataKeyEnumerable::OwnerTokenIdToIndex(to.clone())
                    .get(env)
                    .unwrap_or_else(|| Map::new(env));

            // We set the current token_id with its corresponding index
            token_id_to_index_map.set(token_id, owned_token_indices.len());

            // We push the current created token index to the vetor containing indices of tokens owned
            owned_token_indices.push_back(token_id);

            owner_token_index.set(token_id, owner_token_indices.len());
            owner_token_indices.push_back(token_id);

            DataKeyEnumerable::OwnedTokenIndices.set(env, &owned_token_indices);
            DataKeyEnumerable::TokenIdToIndex.set(env, &token_id_to_index_map);
            DataKeyEnumerable::OwnerOwnedTokenIds(to.clone()).set(env, &owner_token_indices);
            DataKeyEnumerable::OwnerTokenIdToIndex(to.clone()).set(env, &owner_token_index);

            DataKey::Balance(to.clone()).set(env, &owner_token_indices.len());
        } else {
            panic!("Token already exist")
        }

        let mut v: Vec<Val> = Vec::new(env);
        v.push_back(to.into_val(env));
        v.push_back(token_id.into());
        Event::Mint.publish(env, v);
    }
}

#[contractimpl]
impl ERC721 for NotesNFTContract {
    fn balance_of(env: Env, owner: Address) -> u32 {
        DataKey::Balance(owner)
            .extend(&env, 1000)
            .get(&env)
            .unwrap_or(0)
    }

    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, token_id: u32) {
        spender.require_auth();
        let is_sender_approved = if spender != from {
            let has_approved =
                if let Some(approved) = DataKey::Approved(token_id).get::<Address>(&env) {
                    // Clear the approval on transfer
                    DataKey::Approved(token_id).remove(&env);
                    approved == spender
                } else {
                    false
                };
            if !has_approved {
                DataKey::Operator(from.clone(), spender).has(&env)
            } else {
                true
            }
        } else {
            true
        };
        if !is_sender_approved {
            panic_with_error!(&env, Error::NotAuthorized);
        }

        if let Some(addr) = DataKey::TokenOwner(token_id).get::<Address>(&env) {
            if addr == from {
                if from != to {
                    // vector containing ids of tokens owned by a specific address:
                    let from_owned_token_ids_key =
                        DataKeyEnumerable::OwnerOwnedTokenIds(from.clone());
                    let to_owned_token_ids_key = DataKeyEnumerable::OwnerOwnedTokenIds(to.clone());
                    let mut from_owned_token_ids: Vec<u32> = from_owned_token_ids_key
                        .get(&env)
                        .unwrap_or_else(|| Vec::new(&env));

                    // A map linking token IDs to their indices for a specific address.
                    let from_owner_token_id_to_index_key =
                        DataKeyEnumerable::OwnerTokenIdToIndex(from.clone());
                    let to_owner_token_id_to_index_key =
                        DataKeyEnumerable::OwnerTokenIdToIndex(to.clone());
                    let mut from_owner_token_id_to_index: Map<u32, u32> =
                        from_owner_token_id_to_index_key
                            .get(&env)
                            .unwrap_or_else(|| Map::new(&env));

                    let mut to_index: Vec<u32> = to_owned_token_ids_key
                        .get(&env)
                        .unwrap_or_else(|| Vec::new(&env));
                    let mut to_token: Map<u32, u32> = to_owner_token_id_to_index_key
                        .get(&env)
                        .unwrap_or_else(|| Map::new(&env));

                    // Remove token from index of from address
                    if let Some(index) = from_owner_token_id_to_index.get(token_id) {
                        // index is the index for an especific address in
                        if let Some(pos) = from_owned_token_ids.iter().position(|x| x == index) {
                            let pos_u32: u32 = pos.try_into().unwrap();
                            from_owned_token_ids.remove(pos_u32);
                        }
                        from_owner_token_id_to_index.remove(token_id);
                    }

                    // Remove token from index of to address
                    to_token.set(token_id, to_index.len());
                    to_index.push_back(token_id);

                    // Update from address vec and map
                    from_owned_token_ids_key.set(&env, &from_owned_token_ids);
                    from_owner_token_id_to_index_key.set(&env, &from_owner_token_id_to_index);
                    DataKey::Balance(from.clone()).set(&env, &from_owned_token_ids.len());

                    // Update to address vec and map
                    to_owner_token_id_to_index_key.set(&env, &to_token);
                    to_owned_token_ids_key.set(&env, &to_index);
                    DataKey::Balance(to.clone()).set(&env, &to_index.len());

                    // Emit the transfer event
                    let mut v: Vec<Val> = Vec::new(&env);
                    v.push_back(from.clone().into_val(&env));
                    v.push_back(to.into_val(&env));
                    v.push_back(token_id.into());
                    Event::Transfer.publish(&env, v);
                }
                DataKey::TokenOwner(token_id).set(&env, &to);
            } else {
                panic_with_error!(&env, Error::NotOwner);
            }
        } else {
            panic_with_error!(&env, Error::NotNFT);
        }
    }

    fn approve(env: Env, caller: Address, operator: Option<Address>, token_id: u32, ttl: u32) {
        if let Some(owner) = DataKey::TokenOwner(token_id).get::<Address>(&env) {
            if owner == caller {
                owner.require_auth();
            } else if DataKey::Operator(owner, caller.clone())
                .get::<bool>(&env)
                .unwrap_or(false)
            {
                caller.require_auth();
            }
        } else {
            panic_with_error!(&env, Error::NotNFT);
        }

        if let Some(to_approve) = operator {
            DataKey::Approved(token_id).set(&env, &to_approve);
            DataKey::Approved(token_id).extend(&env, ttl);

            // Emit the Approved event
            let mut v: Vec<Val> = Vec::new(&env);
            v.push_back(
                DataKey::TokenOwner(token_id)
                    .get::<Address>(&env)
                    .unwrap()
                    .into_val(&env),
            );
            v.push_back(to_approve.into_val(&env));
            v.push_back(token_id.into());
            Event::Approve.publish(&env, v);
        } else {
            DataKey::Approved(token_id).remove(&env);
        }
    }

    fn set_approval_for_all(
        env: Env,
        caller: Address,
        owner: Address,
        operator: Address,
        approved: bool,
        ttl: u32,
    ) {
        if owner == caller {
            owner.require_auth();
        } else if DataKey::Operator(owner.clone(), caller.clone())
            .get::<bool>(&env)
            .unwrap_or(false)
        {
            caller.require_auth();
        } else {
            panic_with_error!(&env, Error::NotAuthorized);
        }
        let key = DataKey::Operator(owner.clone(), operator.clone());

        if approved {
            key.set(&env, &true);
            key.extend(&env, ttl);
        } else {
            key.remove(&env);
        }

        // Emit the ApprovedForAll event
        let mut v: Vec<Val> = Vec::new(&env);
        v.push_back(owner.into_val(&env));
        v.push_back(operator.into_val(&env));
        v.push_back(approved.into());
        Event::ApproveForAll.publish(&env, v);
    }

    fn get_approved(env: Env, token_id: u32) -> Option<Address> {
        DataKey::Approved(token_id).get(&env).unwrap_or(None)
    }

    fn is_approval_for_all(env: Env, owner: Address, operator: Address) -> bool {
        DataKey::Operator(owner, operator)
            .get(&env)
            .unwrap_or(false)
    }
}

#[contractimpl]
impl ERC721Metadata for NotesNFTContract {
    fn name(env: Env) -> String {
        DatakeyMetadata::Name.get(&env).unwrap()
    }

    fn symbol(env: Env) -> String {
        DatakeyMetadata::Symbol.get(&env).unwrap()
    }

    fn token_uri(env: Env, token_id: u32) -> String {
        if !DataKey::TokenOwner(token_id).has(&env) {
            panic_with_error!(&env, Error::NotNFT);
        }

        DatakeyMetadata::Uri(token_id)
            .get(&env)
            .unwrap_or_else(|| String::from_str(&env, "no uri"))
    }
}

#[contractimpl]
impl ERC721Enumerable for NotesNFTContract {
    fn total_supply(env: Env) -> u32 {
        DataKeyEnumerable::OwnedTokenIndices
            .get::<Vec<u32>>(&env)
            .unwrap()
            .len()
    }

    fn token_by_index(env: Env, index: u32) -> u32 {
        DataKeyEnumerable::OwnedTokenIndices
            .get::<Vec<u32>>(&env)
            .unwrap()
            .get(index)
            .unwrap_or_else(|| panic_with_error!(&env, Error::OutOfBounds))
    }

    fn token_of_owner_by_index(env: Env, owner: Address, index: u32) -> u32 {
        DataKeyEnumerable::OwnerOwnedTokenIds(owner)
            .get::<Vec<u32>>(&env)
            .unwrap_or_else(|| panic_with_error!(&env, Error::OutOfBounds))
            .get(index)
            .unwrap_or_else(|| panic_with_error!(&env, Error::OutOfBounds))
    }
}
