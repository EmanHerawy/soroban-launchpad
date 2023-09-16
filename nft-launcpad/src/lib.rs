#![no_std]
 
 use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, String,Val, Vec,BytesN,IntoVal};
mod nft_721 {
    soroban_sdk::contractimport!(
        file = "../ERC721/target/wasm32-unknown-unknown/release/nft_721.wasm"
    );
}


#[contract]
pub struct NFTTokenlanchpad;
#[contracttype]
#[derive(Clone)]
enum DataKey {
    // key for address array
    DeployedContracts,
    // ispaused
    IsPaused,
    Admin,
}
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    Paused = 1,
    UnAuthorized = 2,
    UnPaused = 3,
}

#[contractimpl]
impl NFTTokenlanchpad {
    pub fn deploy(
        env: Env,
        deployer: Address,
        salt: BytesN<32>,
        admin: Address, base_uri: String, name: String, symbol: String
    ) -> Result<Address, Error>  {
        if  Self::is_paused(env.clone())==true {Err(Error::Paused) }
        else{
             // Skip authorization if deployer is the current contract.
        if deployer != env.current_contract_address() {
            deployer.require_auth();
        }
        let wasm_hash = env
            .deployer()
            .upload_contract_wasm(nft_721::WASM);

        // Deploy the contract using the uploaded Wasm with given hash.
        let deployed_contract = env
            .deployer()
            .with_address(deployer, salt)
            .deploy(wasm_hash);

       
        nft_721::Client::new(&env, &deployed_contract).initialize(
           &admin, &base_uri, &name, &symbol  );
        // Return the contract ID of the deployed contract 
        // get the array from storage Vec::<Address> 
        let mut contracts  = & mut env.storage().instance().get::<_,Vec::<Address> >(&DataKey::DeployedContracts).unwrap_or(Vec::new(&env));      // save address to adress mappings
         contracts.push_back(deployed_contract.clone());
        //  // save to ledger
        env.storage()
            .instance()
            .set(&DataKey::DeployedContracts, contracts);

      Ok( deployed_contract)
    
        }
       
       }

    pub fn is_paused(env: Env) -> bool {
        env.storage()
            .persistent()
            .get::<_, bool>(&DataKey::IsPaused)
            .unwrap()
    }
    pub fn get_admin(env: Env) -> Address {
        env.storage()
            .persistent()
            .get::<_, Address>(&DataKey::Admin)
            .unwrap()
    }
    pub fn set_admin(env: Env, admin: Address) {
        // check that the caller is the old admin
        Self::get_admin(env.clone()).require_auth();

        env.storage().persistent().set(&DataKey::Admin, &admin);
    }
    pub fn pause(env: Env) -> Result<bool, Error> {
        if Self::is_paused(env.clone())==true{
            Err(Error::Paused)
        } else {
            // check that the caller is the old admin
            Self::get_admin(env.clone()).require_auth();

            env.storage().persistent().set(&DataKey::IsPaused, &true);
            Ok(true)
        }
    }
    pub fn unpause(env: Env) -> Result<bool, Error> {
        if Self::is_paused(env.clone())==false {
            Err(Error::UnPaused)
        } else {
            // check that the caller is the old admin
            Self::get_admin(env.clone()).require_auth();

            env.storage().persistent().set(&DataKey::IsPaused, &false);
            Ok(false)
        }
    }

    /************** constructor *********************  */
    pub fn init(env: Env, admin: Address) {
        if env.storage().persistent().has(&DataKey::Admin) {
            panic!("owner is already set");
        }
        env.storage().persistent().set(&DataKey::IsPaused, &false);
        env.storage().persistent().set(&DataKey::Admin, &admin);
    }
}

//  mod test;
