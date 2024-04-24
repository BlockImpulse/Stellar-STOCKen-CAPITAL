use crate::storage;
use soroban_sdk::{contracttype, Address, Env, IntoVal, String, TryFromVal, Val};

#[contracttype]
pub enum DataKey {
    Admin,
    RegisterCounter,
    SignaturitProcess(String),
    OracleProcess(u32),
}

impl storage::Storage for DataKey {
    fn get<V: TryFromVal<Env, Val>>(&self, env: &Env) -> Option<V> {
        storage::Persistent::get(env, self)
    }

    fn set<V: IntoVal<Env, Val>>(&self, env: &Env, val: &V) {
        storage::Persistent::set(env, self, val)
    }

    fn has(&self, env: &Env) -> bool {
        storage::Persistent::has(env, self)
    }

    fn extend(&self, env: &Env, min_ledger_to_live: u32) -> &Self {
        if !self.has(env) {
            return self;
        }

        storage::Persistent::extend(env, self, min_ledger_to_live);

        self
    }

    fn remove(&self, env: &Env) {
        storage::Persistent::remove(env, self)
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignaturitProcess {
    /**
     * The transaction escrow ID is the signature ID (uuid) from Signaturit.
     * It is an UUID (36 bytes length), for example: '6f6c974e-2910-11e4-b3d4-0aa7697eb409'
     */
    pub id: String,
    /**
     * Oracle indentifier for this process
     */
    pub oracle_id: u32,
    /**
    The address where the callback response will be sent to.
     */
    pub send_to: Address,

    pub status: SignatureResponse,
}

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum SignatureResponse {
    Failed = 0,
    Completed = 1,
    Wait = 2,
}
