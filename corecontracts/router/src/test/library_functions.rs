use soroban_sdk::{vec};

use crate::test::{RaumFiRouterTest};
use crate::test::add_liquidity::add_liquidity;
use crate::error::RouterErrorsForLibrary;



// router_quote

#[test]
fn test_quote() {
    let test = RaumFiRouterTest::setup();
    assert_eq!(2,test.contract.router_quote(&1, &100, &200));
    assert_eq!(1,test.contract.router_quote(&2, &200, &100));
}

#[test]
fn test_quote_insufficient_amount() {
    let test = RaumFiRouterTest::setup();
    let result = test.contract.try_router_quote(&0, &100, &200);
    assert_eq!(result, Err(Ok(RouterErrorsForLibrary::LibraryInsufficientAmount)));
}

#[test]
fn test_quote_insufficient_liquidity_0() {
    let test = RaumFiRouterTest::setup();
    let result = test.contract.try_router_quote(&1, &0, &200);
    assert_eq!(result, Err(Ok(RouterErrorsForLibrary::LibraryInsufficientLiquidity)));
}

#[test]
fn test_quote_insufficient_liquidity_1() {
    let test = RaumFiRouterTest::setup();
    let result = test.contract.try_router_quote(&1, &100, &0);
    assert_eq!(result, Err(Ok(RouterErrorsForLibrary::LibraryInsufficientLiquidity)));
}

// router_get_amount_out


#[test]
fn test_get_amount_out() {
    let test = RaumFiRouterTest::setup();
    assert_eq!(1,test.contract.router_get_amount_out(&3, &100, &100));
}
#[test]
fn try_router_get_amount_out_insufficient_input_amount() {
    let test = RaumFiRouterTest::setup();
    let result = test.contract.try_router_get_amount_out(&0, &100, &100);
    assert_eq!(result, Err(Ok(RouterErrorsForLibrary::LibraryInsufficientInputAmount)));
}

#[test]
fn try_router_get_amount_out_insufficient_liquidity_0() {
    let test = RaumFiRouterTest::setup();
    let result = test.contract.try_router_get_amount_out(&2, &0, &100);
    assert_eq!(result, Err(Ok(RouterErrorsForLibrary::LibraryInsufficientLiquidity)));
}

#[test]
fn try_router_get_amount_out_insufficient_liquidity_1() {
    let test = RaumFiRouterTest::setup();
    let result = test.contract.try_router_get_amount_out(&2, &100, &0);
    assert_eq!(result, Err(Ok(RouterErrorsForLibrary::LibraryInsufficientLiquidity)));
}


// router_get_amount_in

    
#[test]
fn test_get_amount_in() {
    let test = RaumFiRouterTest::setup();
    assert_eq!(3,test.contract.router_get_amount_in(&1, &100, &100));
}

#[test]
fn try_router_get_amount_in_insufficient_output_amount() {
    let test = RaumFiRouterTest::setup();
    let result = test.contract.try_router_get_amount_in(&0, &100, &100);
    assert_eq!(result, Err(Ok(RouterErrorsForLibrary::LibraryInsufficientOutputAmount)));
}

#[test]
fn try_router_get_amount_in_insufficient_liquidity_0() {
    let test = RaumFiRouterTest::setup();
    let result = test.contract.try_router_get_amount_in(&1, &0, &100);
    assert_eq!(result, Err(Ok(RouterErrorsForLibrary::LibraryInsufficientLiquidity)));
}

#[test]
fn try_router_get_amount_in_insufficient_liquidity_1() {
    let test = RaumFiRouterTest::setup();
    let result = test.contract.try_router_get_amount_in(&1, &100, &0);
    assert_eq!(result, Err(Ok(RouterErrorsForLibrary::LibraryInsufficientLiquidity)));
}


// router_get_amounts_out


#[test]
fn try_router_get_amounts_out_invalid_path() {
    let test = RaumFiRouterTest::setup();
    test.contract.initialize(&test.factory.address);
    let path = vec![&test.env, test.token_0.address];
    let result = test.contract.try_router_get_amounts_out(&2, &path);
    assert_eq!(result, Err(Ok(RouterErrorsForLibrary::LibraryInvalidPath)));
}

#[test]
fn test_get_amounts_out_not_yet_initialized() {
    let test = RaumFiRouterTest::setup();   
    let path = vec![&test.env, test.token_0.address, test.token_1.address];
    let result = test.contract.try_router_get_amounts_out(&2, &path);
    assert_eq!(result, Err(Ok(RouterErrorsForLibrary::RouterNotInitialized)));
}


#[test]
fn test_get_amounts_out() {
    let test = RaumFiRouterTest::setup();
    
    // TODO: Get rid of this hack?
    test.env.budget().reset_unlimited();

    test.contract.initialize(&test.factory.address);

    let amount_0: i128 = 10_000;
    let amount_1: i128 = 10_000;

    add_liquidity(&test, &amount_0, &amount_1);

    let path = vec![&test.env, test.token_0.address, test.token_1.address];
    assert_eq!(vec![&test.env,3, 1], test.contract.router_get_amounts_out(&3, &path));
}



// router_get_amounts_in

#[test]
fn try_router_get_amounts_in_invalid_path() {
    let test = RaumFiRouterTest::setup();
    test.contract.initialize(&test.factory.address);
    let path = vec![&test.env, test.token_0.address];
    let result = test.contract.try_router_get_amounts_in(&1, &path);
    assert_eq!(result, Err(Ok(RouterErrorsForLibrary::LibraryInvalidPath)));
}

#[test]
fn test_get_amounts_in_not_yet_initialized() {
    let test = RaumFiRouterTest::setup();   
    let path = vec![&test.env, test.token_0.address, test.token_1.address];
    let result = test.contract.try_router_get_amounts_in(&1, &path);
    assert_eq!(result, Err(Ok(RouterErrorsForLibrary::RouterNotInitialized)));
}


#[test]
fn test_get_amounts_in() {
    let test = RaumFiRouterTest::setup();
    
    // TODO: Get rid of this hack?
    test.env.budget().reset_unlimited();

    test.contract.initialize(&test.factory.address);

    let amount_0: i128 = 10_000;
    let amount_1: i128 = 10_000;

    add_liquidity(&test, &amount_0, &amount_1);

    let path = vec![&test.env, test.token_0.address, test.token_1.address];
    assert_eq!(vec![&test.env,3, 1], test.contract.router_get_amounts_in(&1, &path));
}





    
