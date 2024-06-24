use crate::test::{RaumFiPairTest};
use soroban_sdk::{testutils::{Ledger} , String};
use crate::test::pair::PairError;

    
// Pub function that will be used in other tests:

pub fn add_liquidity(test: &RaumFiPairTest, amount_0: &i128, amount_1: &i128) -> i128 {
    
    // User needs to send these tokens first to the contract
    test.token_0.transfer(&test.user, &test.contract.address, &amount_0);
    test.token_1.transfer(&test.user, &test.contract.address, &amount_1);
    test.contract.deposit(&test.user)
}

#[test]
// #[should_panic(expected = "RaumFiPairTest: insufficient amount of token 1 sent")]
fn deposit_only_token_0_sent() {
    let test = RaumFiPairTest::setup();
    test.env.budget().reset_unlimited();
    let amount_0: i128 = 1_000_000;
    test.contract.initialize(&test.factory.address, &test.token_0.address, &test.token_1.address);
    test.token_0.transfer(&test.user, &test.contract.address, &amount_0);
    let res = test.contract.try_deposit(&test.user);
    assert_eq!(res, Err(Ok(PairError::DepositInsufficientAmountToken1)));
}

#[test]
// #[should_panic(expected = "RaumFiPairTest: insufficient first liquidity minted")]
fn deposit_insufficient_first_liquidity() {
    let test = RaumFiPairTest::setup();
    test.env.budget().reset_unlimited();
    let amount_0: i128 = 1_00;
    let amount_1: i128 = 1_00;
    test.contract.initialize(&test.factory.address, &test.token_0.address, &test.token_1.address);
    test.token_0.transfer(&test.user, &test.contract.address, &amount_0);
    test.token_1.transfer(&test.user, &test.contract.address, &amount_1);
    let res = test.contract.try_deposit(&test.user);
    assert_eq!(res, Err(Ok(PairError::DepositInsufficientFirstLiquidity)));
}



#[test]
fn deposit_sufficient_first_liquidity() {
    let test = RaumFiPairTest::setup();
    test.env.budget().reset_unlimited();
    
    let amount_0: i128 = 1_001; //
    let amount_1: i128 = 1_001; //
    test.contract.initialize(&test.factory.address, &test.token_0.address, &test.token_1.address);
    test.token_0.transfer(&test.user, &test.contract.address, &amount_0);
    test.token_1.transfer(&test.user, &test.contract.address, &amount_1);
    test.contract.deposit(&test.user);
}


#[test]
fn deposit_basic() {
    let test = RaumFiPairTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize(&test.factory.address, &test.token_0.address, &test.token_1.address);
    let amount_0 = 1_000_000_000_000_000_000;
    let amount_1 = 4_000_000_000_000_000_000;
    add_liquidity(&test, &amount_0, &amount_1);
}