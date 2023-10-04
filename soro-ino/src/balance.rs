use crate::storage_types::{DataKey, BALANCE_BUMP_AMOUNT,BALANCE_LIFETIME_THRESHOLD,TokenDataKey};
use soroban_sdk::{Address,Vec, Env};

pub fn read_balance(e: &Env, addr: Address) ->Vec<i128> {
    let key = DataKey::Balance(addr);
     e.storage().persistent().get::<DataKey, Vec<i128>>(&key) .unwrap_or(Vec::new(&e))
}
pub fn owner_of(e: &Env, token_id: i128) -> Address {
     let token_key = DataKey::Owners(TokenDataKey{token_id});
   if let Some(owner) = e.storage().persistent().get::<DataKey, Address>(&token_key) {
        e.storage().persistent().bump(&token_key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
        owner
    } else {
        panic!("token not found");
    }}
fn write_owner(e: &Env, addr: Address, token_id: i128) {
    // save tokeid => address
      let token_key = DataKey::Owners(TokenDataKey{token_id});
    e.storage().persistent().set(&token_key, &addr);
     e.storage().persistent().bump(&token_key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}
fn write_balance(e: &Env, addr: Address, token_id: Vec<i128>) {
     // save user => tokenids []
    let key = DataKey::Balance(addr);
     e.storage().persistent().set(&key, &token_id);
     e.storage().persistent().bump(&key,BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}
fn update_counter(e: &Env) ->i128 {
     // save user => tokenids []
    let key = DataKey::Counter;
    let mut counter = e.storage().persistent().get::<DataKey, i128>(&key).unwrap_or(0);
    counter = counter + 1;
     e.storage().persistent().set(&key, &counter);
     e.storage().persistent().bump(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
     counter
}

pub fn receive_balance(e: &Env, addr: Address, token_id: i128) {
    let mut balance = read_balance(e, addr.clone());
    if !is_authorized(e, addr.clone()) {
        panic!("can't receive when deauthorized");
    }
    balance.push_back(token_id);
    write_owner(e, addr.clone(), token_id);
    write_balance(e, addr, balance);
}
pub fn _mint(e: &Env, addr: Address) -> i128 {
    let mut balance = read_balance(e, addr.clone());
    if !is_authorized(e, addr.clone()) {
        panic!("can't receive when deauthorized");
    }
    let token_id = update_counter(e);
    balance.push_back(token_id);
    write_owner(e, addr.clone(), token_id);
    write_balance(e, addr, balance);
    token_id
}

pub fn spend_balance(e: &Env, addr: Address, token_id: i128) {
    let mut balance = read_balance(e, addr.clone());
    if !is_authorized(e, addr.clone()) {
        panic!("can't spend when deauthorized");
    }
       let _= balance.iter()
            .position(|n| n == token_id)
            .map(|e| balance.remove(e.try_into().unwrap()))
            .is_some();


    write_balance(e, addr, balance);
}

pub fn is_authorized(e: &Env, addr: Address) -> bool {
    let key = DataKey::State(addr);
    if let Some(state) = e.storage().persistent().get::<DataKey, bool>(&key) {
        state
    } else {
        true
    }
}

pub fn write_authorization(e: &Env, addr: Address, is_authorized: bool) {
    let key = DataKey::State(addr);
    e.storage().persistent().set(&key, &is_authorized);
}
