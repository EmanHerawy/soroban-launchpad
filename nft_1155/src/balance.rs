use crate::storage_types::{DataKey,BalanceDataKey, BALANCE_BUMP_AMOUNT,TokenDataKey};
use soroban_sdk::{Address,Vec, Env};

pub fn read_balance(e: &Env,category:i128, addr: Address) ->Vec<i128> {
    let key = DataKey::Balance(BalanceDataKey{owner:addr,category});
     e.storage().persistent().get::<DataKey, Vec<i128>>(&key) .unwrap_or(Vec::new(&e))
}
pub fn owner_of(e: &Env,category:i128, token_id: i128) -> Address {
     let toke_key = DataKey::Owners(TokenDataKey{token_id,category});
   if let Some(owner) = e.storage().persistent().get::<DataKey, Address>(&toke_key) {
        e.storage().persistent().bump(&toke_key, BALANCE_BUMP_AMOUNT);
        owner
    } else {
        panic!("token not found");
    }}
fn write_owner(e: &Env, addr: Address,category:i128, token_id: i128) {
    // save tokeid => address
      let toke_key = DataKey::Owners(TokenDataKey{token_id,category});
    e.storage().persistent().set(&toke_key, &addr);
     e.storage().persistent().bump(&toke_key, BALANCE_BUMP_AMOUNT);
}
fn write_balance(e: &Env, addr: Address,category:i128, token_id: Vec<i128>) {
     // save user => tokenids []
    let key = DataKey::Balance(BalanceDataKey{owner:addr,category});
     e.storage().persistent().set(&key, &token_id);
     e.storage().persistent().bump(&key, BALANCE_BUMP_AMOUNT);
}

pub fn receive_balance(e: &Env, addr: Address,category:i128, token_id: i128) {
    let mut balance = read_balance(e,category, addr.clone());
    if !is_authorized(e, addr.clone()) {
        panic!("can't receive when deauthorized");
    }
    balance.push_back(token_id);
    write_owner(e, addr.clone(),category, token_id);
    write_balance(e, addr,category, balance);
}

pub fn spend_balance(e: &Env, addr: Address,category:i128, token_id: i128) {
    let mut balance = read_balance(e,category, addr.clone());
    if !is_authorized(e, addr.clone()) {
        panic!("can't spend when deauthorized");
    }
       let _= balance.iter()
            .position(|n| n == token_id)
            .map(|e| balance.remove(e.try_into().unwrap()))
            .is_some();


    write_balance(e, addr,category, balance);
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
