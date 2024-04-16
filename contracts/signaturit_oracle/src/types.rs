use crate::storage;
use soroban_sdk::{contracttype, Address, Env, IntoVal, String, TryFromVal, Val};

#[contracttype]
pub enum DataKey {
    Admin,
    RegisterCounter,
    SignaturitProcess(u32),
}

impl storage::Storage for DataKey {
    fn get<V: TryFromVal<Env, Val>>(&self, env: &Env) -> Option<V> {
        match self {
            DataKey::Admin | DataKey::RegisterCounter | &DataKey::SignaturitProcess(_) => {
                storage::Persistent::get(env, self)
            }
        }
    }

    fn set<V: IntoVal<Env, Val>>(&self, env: &Env, val: &V) {
        match self {
            DataKey::Admin | DataKey::RegisterCounter | &DataKey::SignaturitProcess(_) => {
                storage::Persistent::set(env, self, val)
            }
        }
    }

    fn has(&self, env: &Env) -> bool {
        match self {
            DataKey::Admin | DataKey::RegisterCounter | &DataKey::SignaturitProcess(_) => {
                storage::Persistent::has(env, self)
            }
        }
    }

    fn extend(&self, env: &Env, min_ledger_to_live: u32) -> &Self {
        if !self.has(env) {
            return self;
        }

        match self {
            DataKey::Admin | DataKey::RegisterCounter | &DataKey::SignaturitProcess(_) => {
                storage::Persistent::extend(env, self, min_ledger_to_live)
            }
        };
        self
    }

    fn remove(&self, env: &Env) {
        match self {
            DataKey::Admin | DataKey::RegisterCounter | &DataKey::SignaturitProcess(_) => {
                storage::Persistent::remove(env, self)
            }
        }
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignaturitProcess {
    pub id: u32,
    /**
     * The transaction escrow ID is the signature ID (uuid) from Signaturit.
     * It is an UUID (36 bytes length), for example: '6f6c974e-2910-11e4-b3d4-0aa7697eb409'
     */
    pub signaturit_id: String,
    /**
    The address where the callback response will be sent to.
     */
    pub send_to: Address,
}
