use soroban_sdk::{symbol_short, Address, Env, Symbol};

pub(crate) fn approve(e: &Env, from: Address, to: Address, token_id: i128, expiration_ledger: u32) {
    let topics = (Symbol::new(e, "approve"), from, to);
    e.events().publish(topics, (token_id, expiration_ledger));
}
pub(crate) fn approve_all(e: &Env, from: Address, to: Address, status:bool,  expiration_ledger: u32) {
    let topics = (Symbol::new(e, "approveAll"), from, to);
    e.events().publish(topics, (status,expiration_ledger));
}

pub(crate) fn transfer(e: &Env, from: Address, to: Address, token_id: i128) {
    let topics = (symbol_short!("transfer"), from, to);
    e.events().publish(topics, token_id);
}

pub(crate) fn mint(e: &Env, admin: Address, to: Address, token_id: i128) {
    let topics = (symbol_short!("mint"), admin, to);
    e.events().publish(topics, token_id);
}

pub(crate) fn clawback(e: &Env, admin: Address, from: Address, token_id: i128) {
    let topics = (symbol_short!("clawback"), admin, from);
    e.events().publish(topics, token_id);
}

pub(crate) fn set_authorized(e: &Env, admin: Address, id: Address, authorize: bool) {
    let topics = (Symbol::new(e, "set_authorized"), admin, id);
    e.events().publish(topics, authorize);
}

pub(crate) fn set_admin(e: &Env, admin: Address, new_admin: Address) {
    let topics = (symbol_short!("set_admin"), admin);
    e.events().publish(topics, new_admin);
}

pub(crate) fn burn(e: &Env, from: Address, token_id: i128) {
    let topics = (symbol_short!("burn"), from);
    e.events().publish(topics, token_id);
}
