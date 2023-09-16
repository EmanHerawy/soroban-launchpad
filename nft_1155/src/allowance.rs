use crate::storage_types::{ ApproveDataKey, DataKey};

use soroban_sdk::{Address, Env};


pub fn read_allowance_all(e: &Env, from: Address, spender: Address) -> bool {
    let key = DataKey::ApproveForAll(ApproveDataKey { from, spender });
    if let Some(allowance) = e.storage().temporary().get::<_, bool>(&key) {
        allowance
    } else {
        panic!("token not found")
    }
}
 


pub fn write_allowance_for_all(
    e: &Env,
    from: Address,
    spender: Address,
    status: bool,
 ) {
    let key = DataKey::ApproveForAll(ApproveDataKey { from, spender });
    e.storage().temporary().set(&key.clone(), &status);
}


