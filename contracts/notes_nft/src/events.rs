use soroban_sdk::{Env, IntoVal, Val};

pub enum Event {
    Mint,
    Transfer,
    Approve,
    Burn,
}
impl Event {
    fn name(&self) -> &'static str {
        match self {
            Event::Mint => stringify!(Mint),
            Event::Transfer => stringify!(Transfer),
            Event::Approve => stringify!(Approve),
            Event::Burn => stringify!(Burn),
        }
    }
    pub fn publish<D>(&self, env: &Env, value: D)
    where
        D: IntoVal<Env, Val>,
    {
        env.events().publish((self.name(),), value);
    }
}
