use soroban_sdk::{
    testutils::{Events, Ledger},
    vec, 
    IntoVal, 
    Symbol,
    Vec,
    Address};
use crate::test::{RaumFiRouterTest};
use crate::test::add_liquidity::add_liquidity;
use crate::event::{
    InitializedEvent,
    AddLiquidityEvent,
    RemoveLiquidityEvent,
    SwapEvent
};


#[test]
fn initialized_event() {
    let test = RaumFiRouterTest::setup();
    test.contract.initialize(&test.factory.address);

    let initialized_event = test.env.events().all().last().unwrap();

    let expected_initialized_event: InitializedEvent = InitializedEvent {
        factory: test.factory.address.clone()
    };

    assert_eq!(
        vec![&test.env, initialized_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("RaumFiRouter", Symbol::new(&test.env,"initialized_router_contract")).into_val(&test.env),
                (expected_initialized_event).into_val(&test.env)
            ),
        ]
    );

    let false_initialized_event: InitializedEvent = InitializedEvent {
        factory: test.user,
    };

    assert_ne!(
        vec![&test.env, initialized_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("RaumFiRouter", Symbol::new(&test.env,"initialized_router_contract")).into_val(&test.env),
                (false_initialized_event).into_val(&test.env)
            ),
        ]
    );


    // Wront symbol_short
    assert_ne!(
        vec![&test.env, initialized_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("RaumFiRouter", Symbol::new(&test.env,"iniit")).into_val(&test.env),
                (expected_initialized_event).into_val(&test.env)
            ),
        ]
    );

    // Wront string
    assert_ne!(
        vec![&test.env, initialized_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address,
                ("RaumFiRouterr", Symbol::new(&test.env,"initialized_router_contract")).into_val(&test.env),
                (expected_initialized_event).into_val(&test.env)
            ),
        ]
    );

}



#[test]
fn add_liquidity_event() {
    let test = RaumFiRouterTest::setup();
    test.contract.initialize(&test.factory.address);

    let amount_0: i128 = 1_000_000_000_000_000_000;
    let amount_1: i128 = 4_000_000_000_000_000_000;

    let (deposited_amount_0, 
        deposited_amount_1, 
        received_liquidity) =add_liquidity(&test, &amount_0, &amount_1);
    let deterministic_pair_address = test.contract.router_pair_for(&test.token_0.address, &test.token_1.address);


    let add_liquidity_event = test.env.events().all().last().unwrap();

    let expected_add_liquidity_event: AddLiquidityEvent = AddLiquidityEvent {
        token_a: test.token_0.address.clone(),
        token_b: test.token_1.address.clone(),
        pair: deterministic_pair_address.clone(),
        amount_a: deposited_amount_0.clone(),
        amount_b: deposited_amount_1.clone(),
        liquidity: received_liquidity,
        to: test.user.clone(),
    };

    assert_eq!(
        vec![&test.env, add_liquidity_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("RaumFiRouter", Symbol::new(&test.env,"add_liquidity_event")).into_val(&test.env),
                (expected_add_liquidity_event).into_val(&test.env)
            ),
        ]
    );

    let false_add_liquidity_event: AddLiquidityEvent = AddLiquidityEvent {
        token_a: test.token_0.address.clone(),
        token_b: test.token_1.address.clone(),
        pair: deterministic_pair_address,
        amount_a: deposited_amount_0.clone(),
        amount_b: deposited_amount_1.clone(),
        liquidity: 0, // False value
        to: test.user.clone(),
    };

    assert_ne!(
        vec![&test.env, add_liquidity_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("RaumFiRouter", Symbol::new(&test.env,"add_liquidity_event")).into_val(&test.env),
                (false_add_liquidity_event).into_val(&test.env)
            ),
        ]
    );

    // Wrong symbol_short
    assert_ne!(
        vec![&test.env, add_liquidity_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("RaumFiRouter", Symbol::new(&test.env,"addd")).into_val(&test.env),
                (expected_add_liquidity_event).into_val(&test.env)
            ),
        ]
    );

    // Wrong string
    assert_ne!(
        vec![&test.env, add_liquidity_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address,
                ("RaumFiRouterr", Symbol::new(&test.env,"add_liquidity_event")).into_val(&test.env),
                (expected_add_liquidity_event).into_val(&test.env)
            ),
        ]
    );
}

#[test]
fn swap_exact_tokens_for_tokens_event() {
    let test = RaumFiRouterTest::setup();
    test.contract.initialize(&test.factory.address);
    let deadline: u64 = test.env.ledger().timestamp() + 1000;  

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    let amount_0: i128 = 1_000_000_000_000_000_000;
    let amount_1: i128 = 4_000_000_000_000_000_000;

    add_liquidity(&test, &amount_0, &amount_1);

    let amount_in = 1_000_000;

    let expected_amount_out = 3987999;

    test.env.budget().reset_unlimited();
    let executed_amounts = test.contract.swap_exact_tokens_for_tokens(
        &amount_in, //amount_in
        &(expected_amount_out),  // amount_out_min
        &path, // path
        &test.user, // to
        &deadline); // deadline

    assert_eq!(executed_amounts.get(0).unwrap(), amount_in);
    assert_eq!(executed_amounts.get(1).unwrap(), expected_amount_out);

    let swap_event = test.env.events().all().last().unwrap();

    let expected_swap_event: SwapEvent = SwapEvent {
        path: path.clone(),
        amounts: executed_amounts.clone(),
        to: test.user.clone(),
    };

    assert_eq!(
        vec![&test.env, swap_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("RaumFiRouter", Symbol::new(&test.env,"swap_tokens_event")).into_val(&test.env),
                (expected_swap_event).into_val(&test.env)
            ),
        ]
    );


    let mut false_path: Vec<Address> = Vec::new(&test.env);
    false_path.push_back(test.token_1.address.clone());
    false_path.push_back(test.token_0.address.clone());


    let false_swap_event: SwapEvent = SwapEvent {
        path: false_path.clone(),
        amounts: executed_amounts.clone(),
        to: test.user.clone(),
    };

    assert_ne!(
        vec![&test.env, swap_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("RaumFiRouter", Symbol::new(&test.env,"swap_tokens_event")).into_val(&test.env),
                (false_swap_event).into_val(&test.env)
            ),
        ]
    );

    // Wrong symbol_short
    assert_ne!(
        vec![&test.env, swap_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("RaumFiRouter", Symbol::new(&test.env,"swape")).into_val(&test.env),
                (expected_swap_event).into_val(&test.env)
            ),
        ]
    );

    // Wrong string
    assert_ne!(
        vec![&test.env, swap_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address,
                ("RaumFiRouterr", Symbol::new(&test.env,"swap_tokens_event")).into_val(&test.env),
                (expected_swap_event).into_val(&test.env)
            ),
        ]
    );
    
}



#[test]
fn swap_tokens_for_exact_tokens_event() {
    let test = RaumFiRouterTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize(&test.factory.address);
    let deadline: u64 = test.env.ledger().timestamp() + 1000;  

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    let amount_0: i128 = 1_000_000_000;
    let amount_1: i128 = 4_000_000_000;

    add_liquidity(&test, &amount_0, &amount_1);

    let expected_amount_out = 5_000_000;
    let amount_in_should = test.contract.router_get_amounts_in(&expected_amount_out, &path).get(0).unwrap();

    let executed_amounts = test.contract.swap_tokens_for_exact_tokens(
        &expected_amount_out, //amount_out
        &(amount_in_should),  // amount_in_max
        &path, // path
        &test.user, // to
        &deadline); // deadline

    let swap_event = test.env.events().all().last().unwrap();

    let expected_swap_event: SwapEvent = SwapEvent {
        path: path.clone(),
        amounts: executed_amounts.clone(),
        to: test.user.clone(),
    };

    assert_eq!(
        vec![&test.env, swap_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("RaumFiRouter", Symbol::new(&test.env,"swap_tokens_event")).into_val(&test.env),
                (expected_swap_event).into_val(&test.env)
            ),
        ]
    );


    let mut false_path: Vec<Address> = Vec::new(&test.env);
    false_path.push_back(test.token_1.address.clone());
    false_path.push_back(test.token_0.address.clone());


    let false_swap_event: SwapEvent = SwapEvent {
        path: false_path.clone(),
        amounts: executed_amounts.clone(),
        to: test.user.clone(),
    };

    assert_ne!(
        vec![&test.env, swap_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("RaumFiRouter", Symbol::new(&test.env,"swap_tokens_event")).into_val(&test.env),
                (false_swap_event).into_val(&test.env)
            ),
        ]
    );

    // Wrong symbol_short
    assert_ne!(
        vec![&test.env, swap_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("RaumFiRouter", Symbol::new(&test.env,"swape")).into_val(&test.env),
                (expected_swap_event).into_val(&test.env)
            ),
        ]
    );

    // Wrong string
    assert_ne!(
        vec![&test.env, swap_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address,
                ("RaumFiRouterr", Symbol::new(&test.env,"swap_tokens_event")).into_val(&test.env),
                (expected_swap_event).into_val(&test.env)
            ),
        ]
    );
}