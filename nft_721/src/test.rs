#![cfg(test)]
extern crate std;

use crate::{contract::Token, TokenClient};
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, Env, IntoVal, Symbol,Vec,
};

fn create_token<'a>(e: &Env, admin: &Address) -> TokenClient<'a> {
    let token = TokenClient::new(e, &e.register_contract(None, Token {}));
    token.initialize(admin, &"base uri".into_val(e), &"name".into_val(e), &"symbol".into_val(e));
    token
}

#[test]
fn test() {
    let e = Env::default();
    e.mock_all_auths();
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
    let token = create_token(&e, &admin1);

    token.mint(&user1, &1);
    assert_eq!(
        e.auths(),
        std::vec![(
            admin1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("mint"),
                    (&user1, token1).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(token.balance_of(&user1).get(0).unwrap(), token1);
    token.mint(&user2, &token2);

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

    let admin = Address::random(&e);
    let user1 = Address::random(&e);
    let user2 = Address::random(&e);
    let token = create_token(&e, &admin);

    token.mint(&user1, &1000);
    assert_eq!(token.balance_of(&user1).get(0).unwrap(), 1000);

    token.transfer(&user1, &user2, &1001);
}

#[test]
#[should_panic]
fn transfer_receive_deauthorized() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::random(&e);
    let user1 = Address::random(&e);
    let user2 = Address::random(&e);
    let token = create_token(&e, &admin);

    token.mint(&user1, &1000);
assert_eq!(token.balance_of(&user1).get(0).unwrap(), 1000);
    token.set_authorized(&user2, &false);
    token.transfer(&user1, &user2, &1);
}

#[test]
#[should_panic]
fn transfer_spend_deauthorized() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::random(&e);
    let user1 = Address::random(&e);
    let user2 = Address::random(&e);
    let token = create_token(&e, &admin);

    token.mint(&user1, &1000);
assert_eq!(token.balance_of(&user1).get(0).unwrap(), 1000);

    token.set_authorized(&user1, &false);
    token.transfer(&user1, &user2, &1);
}

#[test]
#[should_panic]
fn transfer_from_insufficient_allowance() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::random(&e);
    let user1 = Address::random(&e);
    let user2 = Address::random(&e);
    let user3 = Address::random(&e);
    let token = create_token(&e, &admin);

    token.mint(&user1, &1000);
assert_eq!(token.balance_of(&user1).get(0).unwrap(), 1000);

    token.approve(&user1, &user3, &1000); 
    assert_eq!(token.get_approved(&1000), user3);

    token.transfer_from(&user3, &user1, &user2, &101);
}

#[test]
#[should_panic]
fn initialize_already_initialized() {
    let e = Env::default();
    let admin = Address::random(&e);
    let token = create_token(&e, &admin);

    token.initialize(&admin, &"uri".into_val(&e), &"name".into_val(&e), &"symbol".into_val(&e));
}


