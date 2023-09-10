use crate::storage_types::{AllowanceDataKey, ApproveDataKey, DataKey};
use soroban_sdk::{Address, Env};

pub fn read_allowance(e: &Env, token_id: i128) -> Address {
    let key = DataKey::Allowance(AllowanceDataKey { token_id });
    if let Some(allowance) = e.storage().temporary().get::<_, Address>(&key) {
        allowance
    } else {
        panic!("token not found")
    }
}
pub fn read_allowance_all(e: &Env, from: Address, spender: Address) -> bool {
    let key = DataKey::ApproveForAll(ApproveDataKey { from, spender });
    if let Some(allowance) = e.storage().temporary().get::<_, bool>(&key) {
        allowance
    } else {
        panic!("token not found")
    }
}
pub fn remove_allowance(e: &Env, token_id: i128) {
    let key = DataKey::Allowance(AllowanceDataKey { token_id });
    if e.storage().temporary().has(&key) {
        e.storage().temporary().remove(&key);
    }
}

pub fn write_allowance(
    e: &Env,
    spender: Address,
    token_id: i128,
    expiration_ledger: u32,
) {
    
    let key = DataKey::Allowance(AllowanceDataKey { token_id });
    e.storage().temporary().set(&key.clone(), &spender);
}
pub fn write_allowance_for_all(
    e: &Env,
    from: Address,
    spender: Address,
    status: bool,
    expiration_ledger: u32,
) {
    let key = DataKey::ApproveForAll(ApproveDataKey { from, spender });
    e.storage().temporary().set(&key.clone(), &status);
}

pub fn spend_allowance(e: &Env, spender: Address, token_id: i128) {
    let allowed = read_allowance(e, token_id.clone());

if allowed != spender{
    panic!("not allowed")
}
    remove_allowance(e, token_id);
}
