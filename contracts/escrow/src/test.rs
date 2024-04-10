pub mod add_proposal;
pub mod initialize;
pub mod register;

#[cfg(test)]
mod utils {
    use soroban_sdk::{token, Address, Env, String};
    use token::Client as TokenClient;
    use token::StellarAssetClient;

    // keccak256(STOCKEN_ID_1)
    pub const STOCKEN_ID_1: &str =
        "6ef7e237bbddb133bb3504cad9e2ec7ff90c0c9b63567a632dbad8bb2b923728";
    // keccak256(STOCKEN_ID_2)
    pub const STOCKEN_ID_2: &str =
        "af8f0b8ba4749d7a83edcd03a18e3ee3807fca630f8a18e8e59be53ea15c9e95";

    pub fn create_token_contract<'a>(
        e: &Env,
        admin: &Address,
    ) -> (TokenClient<'a>, StellarAssetClient<'a>) {
        let contract_address = e.register_stellar_asset_contract(admin.clone());
        (
            TokenClient::new(e, &contract_address),
            StellarAssetClient::new(e, &contract_address),
        )
    }

    // TODO: Ask for this, the previous way didn't work
    pub fn native_asset_contract_address(e: &Env) -> Address {
        // CDF3YSDVBXV3QU2QSOZ55L4IVR7UZ74HIJKXNJMN4K5MOVFM3NDBNMLY
        let str_address = String::from_str(
            e,
            "CDF3YSDVBXV3QU2QSOZ55L4IVR7UZ74HIJKXNJMN4K5MOVFM3NDBNMLY",
        );

        Address::from_string(&str_address)
    }
}
