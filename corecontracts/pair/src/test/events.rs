extern crate std;
use crate::test::{RaumFiPairTest};
use crate::event::{DepositEvent, SwapEvent, WithdrawEvent, SyncEvent};
use crate::pair_token::{PairTokenClient};
use crate::test::deposit::add_liquidity;
use soroban_sdk::{testutils::{Ledger, Events}, vec, IntoVal, Symbol};

#[test]
fn deposit_event() {
    let test = RaumFiPairTest::setup();
    test.env.budget().reset_unlimited();
    let amount_0: i128 = 1_001; //
    let amount_1: i128 = 1_001; //
    let expected_liquidity: i128 = 901;
    test.contract.initialize(&test.factory.address, &test.token_0.address, &test.token_1.address);
    test.token_0.transfer(&test.user, &test.contract.address, &amount_0);
    test.token_1.transfer(&test.user, &test.contract.address, &amount_1);
    let executed_liquidity = test.contract.deposit(&test.user);

    assert_eq!(expected_liquidity, executed_liquidity);

    let deposit_event = test.env.events().all().last().unwrap();

    let expected_deposit_event: DepositEvent = DepositEvent {
        to: test.user.clone(),
        amount_0: amount_0.clone(),
        amount_1: amount_1.clone(),
        liquidity: expected_liquidity.clone(),
        new_reserve_0: amount_1.clone(),
        new_reserve_1: amount_1.clone()
    };

    assert_eq!(
        vec![&test.env, deposit_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("RaumFiPair", Symbol::new(&test.env,"deposit_pair_event")).into_val(&test.env),
                (expected_deposit_event).into_val(&test.env)
            ),
        ]
    );

    let false_deposit_event: DepositEvent = DepositEvent {
        to: test.user,
        amount_0: 0,
        amount_1: amount_1,
        liquidity: expected_liquidity,
        new_reserve_0: amount_1,
        new_reserve_1: amount_1
    };

    assert_ne!(
        vec![&test.env, deposit_event],
        vec![
            &test.env,
            (
                test.contract.address,
                ("RaumFiPair", Symbol::new(&test.env,"deposit_pair_event")).into_val(&test.env),
                (false_deposit_event).into_val(&test.env)
            ),
        ]
    );

}


#[test]
fn swap_event() {
    let test = RaumFiPairTest::setup();
    test.env.budget().reset_unlimited();

    let amount_0: i128 = 50_000_000;
    let amount_1: i128 = 100_000_000;
    test.contract.initialize(&test.factory.address, &test.token_0.address, &test.token_1.address);
    add_liquidity(&test, &amount_0, &amount_1);

    let init_time = 12345;
    test.env.ledger().with_mut(|li| {
        li.timestamp = init_time;
    });

    let swap_amount_0: i128 = 10_000_000;
    let expected_output_amount_1: i128 = 16624979;

    // The user sends the token first:
    test.token_0.transfer(&test.user, &test.contract.address, &swap_amount_0);
    test.contract.swap(&0, &expected_output_amount_1, &test.user);
    
    let swap_event = test.env.events().all().last().unwrap();

    let expected_swap_event: SwapEvent = SwapEvent {
        to: test.user.clone(),
        amount_0_in: swap_amount_0.clone(),
        amount_1_in: 0,
        amount_0_out: 0,
        amount_1_out: expected_output_amount_1.clone(),
    };

    assert_eq!(
        vec![&test.env, swap_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("RaumFiPair", Symbol::new(&test.env,"swap_pair_event")).into_val(&test.env),
                (expected_swap_event).into_val(&test.env)
            ),
        ]
    );

    let false_swap_event: SwapEvent = SwapEvent {
        to: test.user,
        amount_0_in: swap_amount_0,
        amount_1_in: 1,
        amount_0_out: 0,
        amount_1_out: expected_output_amount_1,
    };

    assert_ne!(
        vec![&test.env, swap_event],
        vec![
            &test.env,
            (
                test.contract.address,
                ("RaumFiPair", Symbol::new(&test.env,"swap_pair_event")).into_val(&test.env),
                (false_swap_event).into_val(&test.env)
            ),
        ]
    );
}


#[test]
fn sync_event() {
    let test = RaumFiPairTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize(&test.factory.address, &test.token_0.address, &test.token_1.address);

    let original_0: i128 = test.token_0.balance(&test.user);
    let original_1: i128 = test.token_1.balance(&test.user);
    let amount_0: i128 = 1_000_000;
    let amount_1: i128 = 4_000_000;
    add_liquidity(&test, &amount_0, &amount_1);

    // New balances:
    assert_eq!(test.token_0.balance(&test.user), original_0.checked_sub(amount_0).unwrap());
    assert_eq!(test.token_1.balance(&test.user), original_1.checked_sub(amount_1).unwrap());
    assert_eq!(test.token_0.balance(&test.contract.address), amount_0);
    assert_eq!(test.token_1.balance(&test.contract.address), amount_1);
    assert_eq!(test.contract.get_reserves(), (amount_0, amount_1));

    //extra tokens sent to skim:
    let amount_0_extra: i128 = 123_000_000;
    let amount_1_extra: i128 = 4_586_000;
    test.token_0.transfer(&test.user, &test.contract.address, &amount_0_extra);
    test.token_1.transfer(&test.user, &test.contract.address, &amount_1_extra);
    assert_eq!(test.token_0.balance(&test.contract.address), amount_0 + amount_0_extra);
    assert_eq!(test.token_1.balance(&test.contract.address), amount_1 + amount_1_extra);
    assert_eq!(test.contract.get_reserves(), (amount_0, amount_1));

    test.contract.sync();

    let sync_event = test.env.events().all().last().unwrap();

    let expected_sync_event: SyncEvent = SyncEvent {
        new_reserve_0: (amount_0 + amount_0_extra),
        new_reserve_1: (amount_1 + amount_1_extra),
    };

    assert_eq!(
        vec![&test.env, sync_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("RaumFiPair", Symbol::new(&test.env,"sync_pair_event")).into_val(&test.env),
                (expected_sync_event).into_val(&test.env)
            ),
        ]
    );

}