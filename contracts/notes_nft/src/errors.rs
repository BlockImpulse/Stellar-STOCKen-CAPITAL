use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug)]
pub enum Error {
    NotInit = 0,
    AlreadyInit = 1,
    NotOwner = 2,
    NotNFT = 3,
    NotAuthorized = 4,
    OutOfBounds = 5,
}
