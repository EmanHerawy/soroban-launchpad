//! This contract demonstrates a sample implementation of the Soroban token
//! interface.
use crate::admin::{has_administrator, read_administrator, write_administrator};
use crate::allowance::{read_allowance, spend_allowance, write_allowance, write_allowance_for_all, read_allowance_all};
use crate::balance::{is_authorized, write_authorization};
use crate::balance::{read_balance, receive_balance, spend_balance, owner_of};
use crate::event;
use crate::metadata::{ read_name,read_base_uri, read_symbol, write_metadata};
use crate::storage_types::INSTANCE_BUMP_AMOUNT;
use soroban_sdk::{contract, contractimpl, Address, Env,Vec, String};
use crate::token_utils::TokenMetadata;
use crate::traits::TokenTrait;

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
    fn initialize(e: Env, admin: Address, base_uri: String, name: String, symbol: String) {
        if has_administrator(&e) {
            panic!("already initialized")
        }
        write_administrator(&e, &admin);
       
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
        unimplemented!()
    }

    fn is_approved_for_all(e: Env, from: Address, spender: Address) -> bool {
        e.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        read_allowance_all(&e, from, spender)
    }

    fn approve(e: Env, from: Address, spender: Address, token_id: i128, expiration_ledger: u32) {
        from.require_auth();

        check_nonnegative_token_id(token_id);

        e.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
          if  owner_of(&e, token_id) != from{
            panic!("not owner")
        }
        write_allowance(&e, spender.clone(), token_id, expiration_ledger);
        event::approve(&e, from, spender, token_id, expiration_ledger);
    }
    fn approval_for_all(e: Env, from: Address, spender: Address, status:bool, expiration_ledger: u32) {
        from.require_auth();

 
        e.storage().instance().bump(INSTANCE_BUMP_AMOUNT);

        write_allowance_for_all(&e, from.clone(), spender.clone(), status, expiration_ledger);
        event::approve_all(&e, from, spender,status,  expiration_ledger);
    }

    fn balance_of(e: Env, id: Address) -> Vec<i128> {
        e.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        read_balance(&e, id)
    }

 
    fn authorized(e: Env, id: Address) -> bool {
        e.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
        is_authorized(&e, id)
    }

    fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        check_nonnegative_token_id(amount);

        e.storage().instance().bump(INSTANCE_BUMP_AMOUNT);

        spend_balance(&e, from.clone(), amount);
        receive_balance(&e, to.clone(), amount);
        event::transfer(&e, from, to, amount);
    }

    fn transfer_from(e: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        check_nonnegative_token_id(amount);

        e.storage().instance().bump(INSTANCE_BUMP_AMOUNT);

        spend_allowance(&e,  spender, amount);
        spend_balance(&e, from.clone(), amount);
        receive_balance(&e, to.clone(), amount);
        event::transfer(&e, from, to, amount)
    }



    fn set_authorized(e: Env, id: Address, authorize: bool) {
        let admin = read_administrator(&e);
        admin.require_auth();

        e.storage().instance().bump(INSTANCE_BUMP_AMOUNT);

        write_authorization(&e, id.clone(), authorize);
        event::set_authorized(&e, admin, id, authorize);
    }

    fn mint(e: Env, to: Address, amount: i128) {
        check_nonnegative_token_id(amount);
        let admin = read_administrator(&e);
        admin.require_auth();

        e.storage().instance().bump(INSTANCE_BUMP_AMOUNT);

        receive_balance(&e, to.clone(), amount);
        event::mint(&e, admin, to, amount);
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
