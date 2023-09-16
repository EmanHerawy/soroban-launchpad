
use soroban_sdk::{Address, Env, String,Vec};
 
pub trait TokenTrait {
    /********************** constructor ****************** */
    fn initialize(e: Env,  admin: Address, base_uri: String, name: String, symbol: String);

    /********************** token setter ****************** */

 
    fn approval_for_all(e: Env, from: Address, spender: Address,status:bool );

    fn transfer(e: Env, from: Address, to: Address , category: i128, token_id: i128);

    fn transfer_from(e: Env, spender: Address, from: Address, to: Address , category: i128, token_id: i128);
    fn mint(e: Env, to: Address, category: i128, token_id: i128);

    /********************** token auth ****************** */

    fn authorized(e: Env, id: Address) -> bool;
    fn set_authorized(e: Env, id: Address, authorize: bool);
    fn set_admin(e: Env, new_admin: Address);

    /********************** token getter ****************** */

    fn is_approved_for_all(e: Env, owner: Address, operator: Address) -> bool;
    fn owner_of(e: Env , category: i128, token_id: i128) -> Address;

    fn balance_of(e: Env, category: i128, id: Address) -> Vec<i128>;

    fn name(e: Env) -> String;
    fn token_uri(e: Env, token_id: i128) -> String;
    fn base_uri(e: Env) -> String;
    fn symbol(e: Env) -> String;
}

