use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug)]
pub enum Error {
    NotOwner = 0,
    NotNFT = 1,
    NotAuthorized = 2,
    OutOfBounds = 4,
}
