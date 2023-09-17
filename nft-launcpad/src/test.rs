#![cfg(test)]
extern crate alloc;
extern crate std;

use crate::{NFTTokenlanchpad, NFTTokenlanchpadClient};
use alloc::vec;
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation}, Symbol,String,
    xdr::{self, ContractIdPreimage, ContractIdPreimageFromAddress, CreateContractArgs, Uint256},
    Address, BytesN, Env, IntoVal, Val, Vec,
};


#[test]
fn test_deploy_from_contract() {
   let env = Env::default();
    let admin_account= Address::random(&env);
 env.mock_all_auths();
    let deployer_client = NFTTokenlanchpadClient::new(&env, &env.register_contract(None, NFTTokenlanchpad));
    deployer_client.init(&admin_account);
     // non admin tries to pause the contract 
        deployer_client.pause();
         assert_eq!(
        env.auths(),
        std::vec![(
            admin_account.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    deployer_client.address.clone(),
                    symbol_short!("pause"),
                    ().into_val(&env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
        deployer_client.unpause();
         assert_eq!(
        env.auths(),
        std::vec![(
            admin_account.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    deployer_client.address.clone(),
                    symbol_short!("unpause"),
                    ().into_val(&env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    // 
    // Upload the Wasm to be deployed from the deployer contract.
    // This can also be called from within a contract if needed.
 
    // Deploy contract using deployer, and include an init function to call.
    let salt = BytesN::from_array(&env, &[0; 32]);
 
    // admin,  &"base_uri".into_val(&env), &"name".into_val(e), &"symbol".into_val(e)
    
     deployer_client.deploy(
        &deployer_client.address,
        &salt,
       &admin_account,  &"base_uri".into_val(&env), &"name".into_val(&env), &"symbol".into_val(&env)
    );

     // No authorizations needed - the contract acts as a factory.
    // assert_eq!(env.auths(), vec![]);

    // Invoke contract to check that it is initialized.
    // let client = contract::Client::new(&env, &contract_id);
    // let sum = client.value();
    // assert_eq!(sum, 5);
}
#[test]
#[should_panic]
fn test_contract_config_change() {
    let env = Env::default();
    let admin_account= Address::random(&env);

    let deployer_client = NFTTokenlanchpadClient::new(&env, &env.register_contract(None, NFTTokenlanchpad));
    let result = deployer_client.init(&admin_account);
     // non admin tries to pause the contract 
     deployer_client.pause();

    // Upload the Wasm to be deployed from the deployer contract.
    // This can also be called from within a contract if needed.
 
    // Deploy contract using deployer, and include an init function to call.
    let salt = BytesN::from_array(&env, &[0; 32]);
     env.mock_all_auths();
    let contract_id = deployer_client.deploy(
        &deployer_client.address,
        &salt,
        &admin_account,  &"base_uri".into_val(&env), &"name".into_val(&env), &"symbol".into_val(&env)
    );

    // No authorizations needed - the contract acts as a factory.
    assert_eq!(env.auths(), vec![]);

    // Invoke contract to check that it is initialized.
    // let client = contract::Client::new(&env, &contract_id);
    // let sum = client.value();
    // assert_eq!(sum, 5);
}

// #[test]
// fn test_deploy_from_address() {
//     let env = Env::default();
//     let deployer_client = NFTTokenlanchpadClient::new(&env, &env.register_contract(None, NFTTokenlanchpad));

//     // Upload the Wasm to be deployed from the deployer contract.
//     // This can also be called from within a contract if needed.
 
//     // Define a deployer address that needs to authorize the deployment.
//     let deployer = Address::random(&env);

//     // Deploy contract using deployer, and include an init function to call.
//     let salt = BytesN::from_array(&env, &[0; 32]);
//     let init_fn = symbol_short!("init");
//     let init_fn_args: Vec<Val> = (5u32,).into_val(&env);
//     env.mock_all_auths();
//     let contract_id =
//         deployer_client.deploy(&deployer,  &salt, &init_fn, &init_fn_args);

 
//     let expected_auth = AuthorizedInvocation {
//         // Top-level authorized function is `deploy` with all the arguments.
//         function: AuthorizedFunction::Contract((
//             deployer_client.address,
//             symbol_short!("deploy"),
//             (
//                 deployer.clone(),
               
//                 salt,
//                 init_fn,
//                 init_fn_args,
//             )
//                 .into_val(&env),
//         )),
//         // From `deploy` function the 'create contract' host function has to be
//         // authorized.
//         sub_invocations: vec![AuthorizedInvocation {
//             function: AuthorizedFunction::CreateContractHostFn(CreateContractArgs {
//                 contract_id_preimage: ContractIdPreimage::Address(ContractIdPreimageFromAddress {
//                     address: deployer.clone().try_into().unwrap(),
//                     salt: Uint256([0; 32]),
//                 }),
//                 executable: xdr::ContractExecutable::Wasm(xdr::Hash(wasm_hash.into_val(&env))),
//             }),
//             sub_invocations: vec![],
//         }],
//     };
//     assert_eq!(env.auths(), vec![(deployer, expected_auth)]);

//     // Invoke contract to check that it is initialized.
//     let client = contract::Client::new(&env, &contract_id);
//     let sum = client.value();
//     assert_eq!(sum, 5);
// }
