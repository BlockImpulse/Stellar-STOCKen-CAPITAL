use soroban_sdk::{Env, IntoVal, Val};

pub enum EscrowEvent {
    Initialized,
    NewProposal,
    RegisterEscrow,
    SignedCompleted,
    SignedFailed,
}

impl EscrowEvent {
    pub fn name(&self) -> &'static str {
        match self {
            EscrowEvent::Initialized => stringify!(Initialized),
            EscrowEvent::NewProposal => stringify!(NewProposal),
            EscrowEvent::RegisterEscrow => stringify!(RegisterEscrow),
            EscrowEvent::SignedCompleted => stringify!(SignedCompleted),
            EscrowEvent::SignedFailed => stringify!(SignedFailed),
        }
    }
    pub fn publish<D>(&self, env: &Env, value: D)
    where
        D: IntoVal<Env, Val>,
    {
        env.events().publish((self.name(),), value);
    }
}
