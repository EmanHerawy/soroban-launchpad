#![cfg(test)]
extern crate std;

use crate::{contract::Token, TokenClient};
use soroban_sdk::{
    symbol_short,token,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, Env, IntoVal, Symbol,
};
mod soroban_token_contract {
    soroban_sdk::contractimport!(
        file = "../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm"
    );
}
use soroban_token_contract::Client as PaymentTokenClient;

fn create_nft_token<'a>(e: &Env, admin: &Address, payment_token: Address,sale_time:u64,pre_sale_price:i128,sale_price:i128) -> TokenClient<'a> {
    let token = TokenClient::new(e, &e.register_contract(None, Token {}));
    token.initialize(admin, &"base uri".into_val(e), &"name".into_val(e), &"symbol".into_val(e), &payment_token,&sale_time,&pre_sale_price,&sale_price);
    token
}
fn create_token_contract<'a>(e: &Env, admin: &Address) -> PaymentTokenClient<'a> {
    let contract_address = e.register_stellar_asset_contract(admin.clone());
            PaymentTokenClient::new(e, &contract_address) 
}

#[test]
fn test() {
    let e = Env::default();
    e.mock_all_auths();
    let sale_price=100;
    let pre_sale_price=50;
    let sale_time=e.ledger().timestamp()+1000;
    let payment_token: Address = Address::random(&e);
    let test_amount=1;
    let token1=1;
    let token2=2;
    let token3=3;
    let token4=4;
    let token5=5;
    let admin1 = Address::random(&e);
    let admin2 = Address::random(&e);
    let user1 = Address::random(&e);
    let user2 = Address::random(&e);
    let user3 = Address::random(&e);
    let token = create_nft_token(&e, &admin1, payment_token.clone(),sale_time,pre_sale_price,sale_price);

    token.mint(&user1, &test_amount);
    assert_eq!(
        e.auths(),
        std::vec![(
            admin1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("mint"),
                    (&user1, test_amount).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(token.balance_of(&user1).get(0).unwrap(), token1);
    token.mint(&user2, &test_amount);

    token.approve(&user2, &user3, &token2);
    assert_eq!(
        e.auths(),
        std::vec![(
            user2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("approve"),
                    (&user2, &user3, token2).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.get_approved(&token2), user3);

    token.transfer(&user1, &user2, &token1);
    assert_eq!(
        e.auths(),
        std::vec![(
            user1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("transfer"),
                    (&user1, &user2, token1).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.balance_of(&user1).get(0).unwrap_or(0), 0);
    assert_eq!(token.balance_of(&user2).get(0).unwrap(), token2);
    assert_eq!(token.balance_of(&user2).get(1).unwrap(), token1);
 
    token.transfer_from(&user3, &user2, &user1, &token2);
    assert_eq!(
        e.auths(),
        std::vec![(
            user3.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    Symbol::new(&e, "transfer_from"),
                    (&user3, &user2, &user1, token2).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

     assert_eq!(token.balance_of(&user1).get(0).unwrap_or(0), token2);
    assert_eq!(token.balance_of(&user2).get(0).unwrap(), token1);

 

    token.set_admin(&admin2);
    assert_eq!(
        e.auths(),
        std::vec![(
            admin1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("set_admin"),
                    (&admin2,).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    token.set_authorized(&user2, &false);
    assert_eq!(
        e.auths(),
        std::vec![(
            admin2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    Symbol::new(&e, "set_authorized"),
                    (&user2, false).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.authorized(&user2), false);

    token.set_authorized(&user3, &true);
    assert_eq!(token.authorized(&user3), true);

}
 

#[test]
#[should_panic]
fn transfer_insufficient_balance() {
    let e = Env::default();
    e.mock_all_auths();
    let test_amount=5;
    let sale_price=100;
    let pre_sale_price=50;
    let sale_time=e.ledger().timestamp()+1000;
    let payment_token: Address = Address::random(&e);
    let admin = Address::random(&e);
    let user1 = Address::random(&e);
    let user2 = Address::random(&e);
    let token = create_nft_token(&e, &admin, payment_token.clone(),sale_time,pre_sale_price,sale_price);

    token.mint(&user1, &test_amount);
    assert_eq!(token.balance_of(&user1).get(0).unwrap(), 1);

    token.transfer(&user1, &user2, &1001);
}
#[test]
#[should_panic]
fn buy_with_zero_payment() {
    let e = Env::default();
    e.mock_all_auths();
    let test_amount=5;
    let sale_price=100;
    let pre_sale_price=50;
    let sale_time=e.ledger().timestamp()+1000;
    let payment_token: Address = Address::random(&e);
    let admin = Address::random(&e);
    let user1 = Address::random(&e);
    let user2 = Address::random(&e);
    let token = create_nft_token(&e, &admin, payment_token.clone(),sale_time,pre_sale_price,sale_price);

    token.buy(&user1, &test_amount);

}
 #[test]
fn buy_token() {
    let e = Env::default();
    e.mock_all_auths();
    let test_amount=5;
    let sale_price=100;
    let pre_sale_price=50;
    let sale_time=e.ledger().timestamp()+1000;
    let admin = Address::random(&e);

    let user1 = Address::random(&e);
    let user2 = Address::random(&e);
        let payment_token =create_token_contract(&e, &admin);

     payment_token.mint(&user1, &1000000);
    let token = create_nft_token(&e, &admin, payment_token.address.clone(),sale_time,pre_sale_price,sale_price);
    payment_token.approve(&user1, &token.address, &1000000, &200);
    token.buy(&user1, &test_amount);

}

#[test]
#[should_panic]
fn transfer_receive_deauthorized() {
    let e = Env::default();
    e.mock_all_auths();
    let test_amount=5;
    let sale_price=100;
    let pre_sale_price=50;
    let sale_time=e.ledger().timestamp()+1000;
    let payment_token: Address = Address::random(&e);
    let admin = Address::random(&e);
    let user1 = Address::random(&e);
    let user2 = Address::random(&e);
    let token = create_nft_token(&e, &admin, payment_token.clone(),sale_time,pre_sale_price,sale_price);

    token.mint(&user1, &test_amount);
assert_eq!(token.balance_of(&user1).get(0).unwrap(), 1);
    token.set_authorized(&user2, &false);
    token.transfer(&user1, &user2, &100);
}

#[test]
#[should_panic]
fn transfer_spend_deauthorized() {
    let e = Env::default();
    e.mock_all_auths();
    let test_amount=5;
    let sale_price=100;

    let pre_sale_price=50;
    let sale_time=e.ledger().timestamp()+1000;
    let payment_token: Address = Address::random(&e);
    let admin = Address::random(&e);
    let user1 = Address::random(&e);
    let user2 = Address::random(&e);
    let token = create_nft_token(&e, &admin, payment_token.clone(),sale_time,pre_sale_price,sale_price);

    token.mint(&user1, &test_amount);
assert_eq!(token.balance_of(&user1).get(0).unwrap(), 1);

    token.set_authorized(&user1, &false);
    token.transfer(&user1, &user2, &100);
}

#[test]
#[should_panic]
fn transfer_from_insufficient_allowance() {
    let e = Env::default();
    e.mock_all_auths();
    let test_amount=5;
    let sale_price=100;
    let pre_sale_price=50;
    let sale_time=e.ledger().timestamp()+1000;
    let payment_token: Address = Address::random(&e);
    let admin = Address::random(&e);
    let user1 = Address::random(&e);
    let user2 = Address::random(&e);
    let user3 = Address::random(&e);
    let token = create_nft_token(&e, &admin, payment_token.clone(),sale_time,pre_sale_price,sale_price);

    token.mint(&user1, &test_amount);
assert_eq!(token.balance_of(&user1).get(0).unwrap(), 1);

    token.approve(&user1, &user3, &1); 
    assert_eq!(token.get_approved(&1), user3);

    token.transfer_from(&user3, &user1, &user2, &101);
}

#[test]
#[should_panic]
fn initialize_already_initialized() {
    let e = Env::default();
    let admin = Address::random(&e);
        let sale_price=100;
    let pre_sale_price=50;
    let sale_time=e.ledger().timestamp()+1000;
    let payment_token: Address = Address::random(&e);
    let token = create_nft_token(&e, &admin, payment_token.clone(),sale_time,pre_sale_price,sale_price);

    token.initialize(&admin, &"uri".into_val(&e), &"name".into_val(&e), &"symbol".into_val(&e),&payment_token,&sale_time,&pre_sale_price,&sale_price);
}


