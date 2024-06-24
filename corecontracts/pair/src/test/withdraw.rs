use crate::test::{RaumFiPairTest};
use crate::test::deposit::add_liquidity;
use crate::test::pair::PairError;

#[test]
// #[should_panic(expected = "RaumFiPair: liquidity was not initialized yet")]
fn try_withdraw_not_yet_deposited() {
    let test = RaumFiPairTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize(&test.factory.address, &test.token_0.address, &test.token_1.address);
    let result = test.contract.try_withdraw(&test.user);
    assert_eq!(result, Err(Ok(PairError::WithdrawLiquidityNotInitialized)));
}

#[test]
// #[should_panic(expected = "RaumFiPair: insufficient sent shares")]
fn try_withdraw_not_shares_sent() {
    let test = RaumFiPairTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize(&test.factory.address, &test.token_0.address, &test.token_1.address);
    let amount_0: i128 = 50_000_000;
    let amount_1: i128 = 100_000_000;
    add_liquidity(&test, &amount_0, &amount_1);
    let result = test.contract.try_withdraw(&test.user);
    assert_eq!(result, Err(Ok(PairError::WithdrawInsufficientSentShares)));
}




#[test]
fn withdraw() {
    let test = RaumFiPairTest::setup();    
    test.env.budget().reset_unlimited();
    test.contract.initialize(&test.factory.address, &test.token_0.address, &test.token_1.address);
    let amount_0: i128 = 3_000_000;
    let amount_1: i128 = 3_000_000;
    let expected_liquidity: i128 =  3_000_000;
    let minimum_liquidity: i128 = 1_00;
    add_liquidity(&test, &amount_0, &amount_1);

    test.contract.transfer(&test.user, &test.contract.address, &expected_liquidity.checked_sub(minimum_liquidity).unwrap());

    test.contract.withdraw(&test.user);
    assert_eq!(test.contract.balance(&test.user), 0);
    assert_eq!(test.contract.total_supply(), minimum_liquidity);
    assert_eq!(test.token_0.balance(&test.contract.address), 100);
    assert_eq!(test.token_1.balance(&test.contract.address), 100);

    let original_total_supply_0: i128 = 123_000_000_000_000_000_000; // from the test file
    let original_total_supply_1: i128 = 321_000_000_000_000_000_000; // from the test file

    assert_eq!(test.token_0.balance(&test.user), original_total_supply_0.checked_sub(100).unwrap());
    assert_eq!(test.token_1.balance(&test.user), original_total_supply_1.checked_sub(100).unwrap());

    
}
