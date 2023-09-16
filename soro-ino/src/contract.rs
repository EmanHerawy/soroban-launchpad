//! This contract demonstrates a sample implementation of the Soroban token
//! interface.
use crate::admin::{has_administrator, read_administrator, write_administrator};
use crate::allowance::{read_allowance, spend_allowance, write_allowance, write_allowance_for_all, read_allowance_all};
use crate::balance::{is_authorized, write_authorization};
use crate::balance::{read_balance, receive_balance,_mint, spend_balance, owner_of};
use crate::event;
use crate::metadata::{ read_name,read_base_uri, read_symbol, write_metadata};
use crate::storage_types::INSTANCE_BUMP_AMOUNT;
use soroban_sdk::{contract, contractimpl, Address, Env,Vec, String,IntoVal, FromVal};
use crate::token_utils::TokenMetadata;
use crate::sale::{get_price,write_sale_data, read_payment_token};
use crate::traits::TokenTrait;
mod soroban_token_contract {
    soroban_sdk::contractimport!(
        file = "../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm"
    );
}
fn check_nonnegative_token_id(token_id: i128) {
    /// @notice: no token id =0 allowed
    if token_id < 0 {
        panic!("negative token_id is not allowed: {}", token_id)
    }
}

#[contract]
pub struct Token;

#[contractimpl]
impl TokenTrait for Token {
       fn initialize(e: Env,  admin: Address, base_uri: String, name: String, symbol: String, payment_token: Address, sale_time:u64,pre_sale_price:i128,sale_price:i128) {
        if has_administrator(&e) {
            panic!("already initialized")
        }
        write_administrator(&e, &admin);
       write_sale_data(&e, payment_token, sale_time, pre_sale_price, sale_price);
        write_metadata(
            &e,
            TokenMetadata {
                base_uri,
                 name,
                symbol,
            },
        )
    }

        fn get_approved(e: Env, token_id: i128) -> Address{
              e.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        read_allowance(&e, token_id)
        }
    fn owner_of(e: Env, token_id: i128) -> Address{
          e.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        owner_of(&e, token_id)
    }
    fn token_uri(e: Env, token_id: i128) -> String{
// &<i128 as IntoVal<Env, T>>::into_val(&token_id, &e).to_string() 
        read_base_uri(&e)// + &<i128 as IntoVal<Env, T>>::into_val(&token_id, &e).to_string()
        }

    fn is_approved_for_all(e: Env, from: Address, spender: Address) -> bool {
        e.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        read_allowance_all(&e, from, spender)
    }

    fn approve(e: Env, from: Address, spender: Address, token_id: i128) {
        from.require_auth();

        check_nonnegative_token_id(token_id);

        e.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
          if  owner_of(&e, token_id) != from{
            panic!("not owner")
        }
        write_allowance(&e, spender.clone(), token_id);
        event::approve(&e, from, spender, token_id);
    }
    fn approval_for_all(e: Env, from: Address, spender: Address, status:bool) {
        from.require_auth();

 
        e.storage().instance().bump(INSTANCE_BUMP_AMOUNT);

        write_allowance_for_all(&e, from.clone(), spender.clone(), status);
        event::approve_all(&e, from, spender,status);
    }

    fn balance_of(e: Env, id: Address) -> Vec<i128> {
        e.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        read_balance(&e, id)
    }

 
    fn authorized(e: Env, id: Address) -> bool {
        e.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        is_authorized(&e, id)
    }

    fn transfer(e: Env, from: Address, to: Address, token_id: i128) {
        from.require_auth();

        check_nonnegative_token_id(token_id);
  if  owner_of(&e, token_id) != from{
            panic!("not owner")
        }
        e.storage().instance().bump(INSTANCE_BUMP_AMOUNT);

        spend_balance(&e, from.clone(), token_id);
        receive_balance(&e, to.clone(), token_id);
        event::transfer(&e, from, to, token_id);
    }

    fn transfer_from(e: Env, spender: Address, from: Address, to: Address, token_id: i128) {
        spender.require_auth();

        check_nonnegative_token_id(token_id);

        e.storage().instance().bump(INSTANCE_BUMP_AMOUNT);

        spend_allowance(&e,  spender, token_id);
        spend_balance(&e, from.clone(), token_id);
        receive_balance(&e, to.clone(), token_id);
        event::transfer(&e, from, to, token_id)
    }



    fn set_authorized(e: Env, id: Address, authorize: bool) {
        let admin = read_administrator(&e);
        admin.require_auth();

        e.storage().instance().bump(INSTANCE_BUMP_AMOUNT);

        write_authorization(&e, id.clone(), authorize);
        event::set_authorized(&e, admin, id, authorize);
    }

    fn mint(e: Env, to: Address, amount: i128) {
        let admin = read_administrator(&e);
        admin.require_auth();

        e.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        for _i in 0..amount{
       let token_id= _mint(&e, to.clone());
        check_nonnegative_token_id(token_id);
        event::mint(&e, admin.clone(), to.clone(), token_id);
        }
         
      
    }
    fn buy(e: Env, to: Address, amount: i128) {
        // check sales time to get price of nft 
        let price = get_price(&e) * amount;

        let payment_token =  soroban_token_contract::Client::new(&e, &read_payment_token(&e));
        // transfer price to mint 
        payment_token.transfer_from(&e.current_contract_address(),&to, &e.current_contract_address(), &price);     

        e.storage().instance().bump(INSTANCE_BUMP_AMOUNT);

        for _i in 0..amount{
       let token_id= _mint(&e, to.clone());
        check_nonnegative_token_id(token_id);
        // need to figure out how to get Address::zero() 
        event::mint(&e, e.current_contract_address(), to.clone(), token_id);
        }
    }

    fn set_admin(e: Env, new_admin: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();

        e.storage().instance().bump(INSTANCE_BUMP_AMOUNT);

        write_administrator(&e, &new_admin);
        event::set_admin(&e, admin, new_admin);
    }

 
    fn name(e: Env) -> String {
        read_name(&e)
    }

    fn symbol(e: Env) -> String {
        read_symbol(&e)
    }
    fn base_uri(e: Env) -> String{
        read_base_uri(&e)
    }
}
