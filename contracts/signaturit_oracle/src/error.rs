use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum OracleError {
    NotInit = 0,
    AlreadyInit = 1,
    OnlyAdmin = 2,
    SignatureIdAlredyExist = 3,
    MissingDocHash = 4,
    ProcessNotFound = 5,
}
