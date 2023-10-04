use soroban_sdk::{contracttype, Address,String};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub(crate) const BALANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const BALANCE_LIFETIME_THRESHOLD: u32 = BALANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;


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
 #[contracttype]
pub struct TokenURIValue {
    pub token_id: i128,
    pub base_uri: String,
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
