
use soroban_sdk::{Address, Env, String,Vec};
 
pub trait TokenTrait {
    /********************** constructor ****************** */
    fn initialize(e: Env,  admin: Address, base_uri: String, name: String, symbol: String, payment_token: Address, sale_time:u64,pre_sale_price:i128,sale_price:i128);

    /********************** token setter ****************** */

    fn approve(e: Env, from: Address, spender: Address, token_id: i128);

    fn approval_for_all(e: Env, from: Address, spender: Address,status:bool );

    fn transfer(e: Env, from: Address, to: Address, token_id: i128);

    fn transfer_from(e: Env, spender: Address, from: Address, to: Address, token_id: i128);
    fn buy(e: Env, to: Address, amount: i128);
    fn mint(e: Env, to: Address, token_id: i128);

    /********************** token auth ****************** */

    fn authorized(e: Env, id: Address) -> bool;
    fn set_authorized(e: Env, id: Address, authorize: bool);
    fn set_admin(e: Env, new_admin: Address);

    /********************** token getter ****************** */

    fn is_approved_for_all(e: Env, owner: Address, operator: Address) -> bool;
    fn get_approved(e: Env, token_id: i128) -> Address;
    fn owner_of(e: Env, token_id: i128) -> Address;

    fn balance_of(e: Env, id: Address) -> Vec<i128>;

    fn name(e: Env) -> String;
    fn token_uri(e: Env, token_id: i128) -> String;
    fn base_uri(e: Env) -> String;
    fn symbol(e: Env) -> String;
}

