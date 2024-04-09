pub mod add_proposal;
pub mod initialize;

#[cfg(test)]
mod utils {
    use soroban_sdk::{token, Address, Env, String};
    use token::Client as TokenClient;
    use token::StellarAssetClient;

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
