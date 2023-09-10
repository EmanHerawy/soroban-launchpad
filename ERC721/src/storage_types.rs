use soroban_sdk::{contracttype, Address};

pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 34560; // 2 days
pub(crate) const BALANCE_BUMP_AMOUNT: u32 = 518400; // 30 days

#[derive(Clone)]
#[contracttype]
pub struct AllowanceDataKey {
   pub token_id: i128,
}
#[derive(Clone)]
#[contracttype]
pub struct ApproveDataKey {
    pub from: Address,
    pub spender: Address,
 }
#[derive(Clone)]
#[contracttype]
pub struct TokenDataKey {
    pub token_id: i128
 }


#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    // to which token id 
    Counter,
    Allowance(AllowanceDataKey),
    Owners(TokenDataKey),
    ApproveForAll(ApproveDataKey),
    Balance(Address),
    State(Address),
    Admin,
}
