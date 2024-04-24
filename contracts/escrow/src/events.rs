use soroban_sdk::{vec, Address, Env, IntoVal, String, Val, Vec};

pub enum EscrowEvent2 {
    Initialized(Address, Address, Address),
    NewProposal(String, Address),
    RegisterEscrow(String, String, u32, Address, i128),
    SignedCompleted(String, String, Address, Address, i128, u32),
    SignedFailed(String, String, Address),
}

impl EscrowEvent2 {
    pub fn name(&self) -> &'static str {
        match self {
            EscrowEvent2::Initialized(..) => stringify!(Initialized),
            EscrowEvent2::NewProposal(..) => stringify!(NewProposal),
            EscrowEvent2::RegisterEscrow(..) => stringify!(RegisterEscrow),
            EscrowEvent2::SignedCompleted(..) => stringify!(SignedCompleted),
            EscrowEvent2::SignedFailed(..) => stringify!(SignedFailed),
        }
    }
    pub fn publish(&self, env: &Env) {
        match self {
            EscrowEvent2::Initialized(asset_address, oracle_address, nft_notes_address) => {
                let values: Vec<Val> = vec![
                    env,
                    asset_address.into_val(env),
                    oracle_address.into_val(env),
                    nft_notes_address.into_val(env),
                ];
                self.internal_publish(env, values);
            }
            EscrowEvent2::NewProposal(escrow_id, proposer) => {
                let values: Vec<Val> =
                    vec![env, (*escrow_id).into_val(env), proposer.into_val(env)];
                self.internal_publish(env, values);
            }
            EscrowEvent2::RegisterEscrow(signaturit_id, propose_id, oracle_id, buyer, funds) => {
                let values: Vec<Val> = vec![
                    env,
                    signaturit_id.into_val(env),
                    propose_id.into_val(env),
                    (*oracle_id).into_val(env),
                    buyer.into_val(env),
                    funds.into_val(env),
                ];
                self.internal_publish(env, values);
            }
            EscrowEvent2::SignedCompleted(
                signaturit_id,
                propose_id,
                buyer,
                receiver,
                funds,
                nft_id,
            ) => {
                let values: Vec<Val> = vec![
                    env,
                    signaturit_id.into_val(env),
                    propose_id.into_val(env),
                    buyer.into_val(env),
                    receiver.into_val(env),
                    funds.into_val(env),
                    nft_id.into_val(env),
                ];
                self.internal_publish(env, values);
            }
            EscrowEvent2::SignedFailed(signaturit_id, propose_id, buyer) => {
                let values: Vec<Val> = vec![
                    env,
                    signaturit_id.into_val(env),
                    propose_id.into_val(env),
                    buyer.into_val(env),
                ];
                self.internal_publish(env, values);
            }
        }
    }

    fn internal_publish<D>(&self, env: &Env, value: D)
    where
        D: IntoVal<Env, Val>,
    {
        env.events().publish((self.name(),), value);
    }
}
