use soroban_sdk::{Address, Env, IntoVal, String, Val, Vec};

pub enum OracleEvent {
    Initialized(Address),
    NewSignatureProcess(String, u32),
    SignatureResponse(String, u32, bool),
}

impl OracleEvent {
    pub fn name(&self) -> &'static str {
        match self {
            OracleEvent::Initialized(..) => stringify!(Initialized),
            OracleEvent::NewSignatureProcess(..) => stringify!(NewSignatureProcess),
            OracleEvent::SignatureResponse(..) => stringify!(SignatureResponse),
        }
    }
    pub fn publish(&self, env: &Env) {
        let mut v: Vec<Val> = Vec::new(&env);

        match self {
            OracleEvent::Initialized(admin_address) => {
                v.push_back(admin_address.into_val(env));
            }
            OracleEvent::NewSignatureProcess(signaturit_id, oracle_id) => {
                v.push_back(signaturit_id.into_val(env));
                v.push_back(oracle_id.into_val(env));
            }

            OracleEvent::SignatureResponse(signaturit_id, oracle_id, is_success) => {
                v.push_back(signaturit_id.into_val(env));
                v.push_back(oracle_id.into_val(env));
                v.push_back(is_success.into_val(env));
            }
        }

        env.events().publish((self.name(),), v)
    }
}
