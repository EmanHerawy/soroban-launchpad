use soroban_sdk::{contracttype, Address};

pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 34560; // 2 days
pub(crate) const BALANCE_BUMP_AMOUNT: u32 = 518400; // 30 days

#[derive(Clone)]
#[contracttype]
pub struct BalanceDataKey {
 pub owner: Address,
   pub category: i128,
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
    pub token_id: i128,
     pub category: i128
 }


#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Owners(TokenDataKey),
    ApproveForAll(ApproveDataKey),
    Balance(BalanceDataKey),
    State(Address),
    Admin,
}
