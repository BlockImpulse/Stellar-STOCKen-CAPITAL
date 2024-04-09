use crate::storage;
use soroban_sdk::{
    contracterror, contracttype, Address, Env, IntoVal, String, TryFromVal, Val, U256,
};

#[contracttype]
pub enum DataKey {
    Proposal(String),
    SignatureProcess(String),
}

impl storage::Storage for DataKey {
    fn get<V: TryFromVal<Env, Val>>(&self, env: &Env) -> Option<V> {
        match self {
            DataKey::Proposal(_) => storage::Persistent::get(env, self),
            DataKey::SignatureProcess(_) => storage::Temporary::get(env, self),
        }
    }

    fn set<V: IntoVal<Env, Val>>(&self, env: &Env, val: &V) {
        match self {
            DataKey::Proposal(_) => storage::Persistent::set(env, self, val),
            DataKey::SignatureProcess(_) => storage::Temporary::set(env, self, val),
        }
    }

    fn has(&self, env: &Env) -> bool {
        match self {
            DataKey::Proposal(_) => storage::Persistent::has(env, self),
            DataKey::SignatureProcess(_) => storage::Temporary::has(env, self),
        }
    }

    fn extend(&self, env: &Env, min_ledger_to_live: u32) -> &Self {
        if !self.has(env) {
            return self;
        }

        match self {
            DataKey::Proposal(_) => storage::Persistent::extend(env, self, min_ledger_to_live),
            DataKey::SignatureProcess(_) => {
                storage::Temporary::extend(env, self, min_ledger_to_live)
            }
        };
        self
    }

    fn remove(&self, env: &Env) {
        match self {
            DataKey::Proposal(_) => storage::Persistent::remove(env, self),
            DataKey::SignatureProcess(_) => storage::Temporary::remove(env, self),
        }
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EscrowProposal {
    /**
    Proposal ID from stocken, so can know what is the propose
     */
    pub escrow_id: String,

    /**
    Current status of the proposal
     */
    pub status: ProposalStatus,

    /**
    Owner of the proposal, who will receive the fund after success
     */
    pub owner: Address,

    /**
     * The minimun funds asked by the proposal
     */
    pub min_funds: U256,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignatureTxEscrow {
    /**
     * The transaction escrow ID is the signature ID from Signaturit.
     * It is an UUID (36 bytes length), for example: '6f6c974e-2910-11e4-b3d4-0aa7697eb409'
     */
    pub id: String,
    /**
     *
     */
    pub oracle_id: u32,
    /**
     * Address of the user that pick the propose
     */
    pub buyer: Address,
    /**
     * Address of the owner of the propose and who will receive the funds if success
     */
    pub receiver: Address,
    /**
     * Funds that the seller provide to the propose
     */
    pub funds: U256,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum EscrowError {
    OnlyOwner = 1,
    OnlyOracle = 2,
    AlreadyProposed = 3,
    ProposalNotFound = 4,
    PickedOrCanceled = 5,
    NoEnoughtFunds = 6,
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ProposalStatus {
    Canceled = 0,
    Actived = 1,
    Picked = 2,
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum SignatureStatus {
    Canceled = 0,
    Actived = 1,
    Picked = 2,
}
