use soroban_sdk::{Address, Env};

use crate::storage_types::DataKey;


/*********** read fro ledger********************* */

pub fn read_payment_token(e: &Env) -> Address {
    let key = DataKey::PaymentToken;
    e.storage().instance().get(&key).unwrap()
}
pub fn read_sale_price(e: &Env) -> i128 {
    let key = DataKey::SalePrice;
    e.storage().instance().get(&key).unwrap()
}
pub fn read_pre_sale_price(e: &Env) -> i128 {
    let key = DataKey::PreSalePrice;
    e.storage().instance().get(&key).unwrap()
}
pub fn read_sale_time(e: &Env) -> u64 {
    let key = DataKey::SaleTime;
    e.storage().instance().get(&key).unwrap()
}

pub fn get_price(e: &Env) -> i128 {
  let sale_time = read_sale_time(&e);
  if sale_time > 0 && sale_time < e.ledger().timestamp() {
     read_sale_price(&e)
  } else {
     read_pre_sale_price(&e)

  }
     
}


/********write to ledger***************** */
 fn write_sale_price(e: &Env, sale_price:i128) {
    let key = DataKey::SalePrice;
    e.storage().instance().set(&key,&sale_price);
}
 fn write_pre_sale_price(e: &Env, pre_sale_price:i128) {
    let key = DataKey::PreSalePrice;
    e.storage().instance().set(&key,&pre_sale_price);
}
 fn write_sale_time(e: &Env, sale_time:u64) {
    if sale_time < e.ledger().timestamp() {
        panic!("sale time must be in future");
    }
    let key = DataKey::SaleTime;
    e.storage().instance().set(&key,&sale_time);
}
fn write_payment_token(e: &Env, payment_token: Address) {
    let key = DataKey::PaymentToken;
    e.storage().instance().set(&key,&payment_token);
}
pub fn write_sale_data(e: &Env, payment_token: Address, sale_time:u64,pre_sale_price:i128,sale_price:i128) {
    write_pre_sale_price(&e, pre_sale_price);
    write_payment_token(&e, payment_token);
    write_sale_time(&e, sale_time);
    write_sale_price(&e, sale_price);
}
