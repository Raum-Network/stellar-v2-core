use soroban_sdk::{testutils::{Events}, vec, IntoVal, Symbol};
use soroban_sdk::{xdr::{ToXdr}, Bytes}; // For determinisitic address
use crate::test::{RaumFiFactoryTest};
use crate::event::{
    InitializedEvent,
    PairEvent};


#[test]
fn initialized_event() {
    let test = RaumFiFactoryTest::setup();
    test.contract.initialize(&test.admin, &test.pair_wasm);

    let initialized_event = test.env.events().all().last().unwrap();

    let expected_initialized_event: InitializedEvent = InitializedEvent {
        setter: test.admin.clone()
    };

    assert_eq!(
        vec![&test.env, initialized_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("RaumFiFactory", Symbol::new( &test.env , "initialize_factory_contract")).into_val(&test.env),
                (expected_initialized_event).into_val(&test.env)
            ),
        ]
    );

    let false_initialized_event: InitializedEvent = InitializedEvent {
        setter: test.user,
    };

    assert_ne!(
        vec![&test.env, initialized_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("RaumFiFactory", Symbol::new(&test.env, "initialize_factory_contract")).into_val(&test.env),
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
                ("RaumFiFactory", Symbol::new(&test.env,"iniit")).into_val(&test.env),
                (expected_initialized_event).into_val(&test.env)
            ),
        ]
    );

    // Wrongt string
    assert_ne!(
        vec![&test.env, initialized_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address,
                ("RaumFiFactoryy", Symbol::new(&test.env,"initialize_factory_contract")).into_val(&test.env),
                (expected_initialized_event).into_val(&test.env)
            ),
        ]
    );

}


#[test]
fn new_pair_event() {
    let test = RaumFiFactoryTest::setup();
    test.contract.initialize(&test.admin, &test.pair_wasm);
    test.contract.create_pair(&test.token_0.address, &test.token_1.address);

    // Calculating pair address:
    let mut salt = Bytes::new(&test.env);
    salt.append(&test.token_0.address.clone().to_xdr(&test.env)); 
    salt.append(&test.token_1.address.clone().to_xdr(&test.env));
    let bytes_n_32_salt=test.env.crypto().sha256(&salt);
    let deterministic_pair_address = test.env.deployer().with_address(test.contract.address.clone(), bytes_n_32_salt.clone()).deployed_address();

    let new_pair_event = test.env.events().all().last().unwrap();

    let expected_new_pair_event: PairEvent = PairEvent {
        token_0: test.token_0.address.clone(),
        token_1: test.token_1.address.clone(),
        pair: deterministic_pair_address.clone(),
        new_pairs_length: 1,
    };

    assert_eq!(
        vec![&test.env, new_pair_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("RaumFiFactory", Symbol::new(&test.env,"new_pair_created")).into_val(&test.env),
                (expected_new_pair_event).into_val(&test.env)
            ),
        ]
    );

    let false_new_pair_event: PairEvent = PairEvent {
        token_0: test.token_1.address,
        token_1: test.token_0.address,
        pair: deterministic_pair_address,
        new_pairs_length: 1,
    };

    assert_ne!(
        vec![&test.env, new_pair_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("RaumFiFactory", Symbol::new(&test.env,"new_pair_created")).into_val(&test.env),
                (false_new_pair_event).into_val(&test.env)
            ),
        ]
    );


    // Wront symbol_short
    assert_ne!(
        vec![&test.env, new_pair_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("RaumFiFactory", Symbol::new(&test.env,"new_pairr")).into_val(&test.env),
                (expected_new_pair_event).into_val(&test.env)
            ),
        ]
    );

    // Wront string
    assert_ne!(
        vec![&test.env, new_pair_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("RaumFiFactoryy", Symbol::new(&test.env,"new_pair")).into_val(&test.env),
                (expected_new_pair_event).into_val(&test.env)
            ),
        ]
    );

    // new pair
    test.contract.create_pair(&test.token_2.address, &test.token_3.address);
    // Calculating pair address:
    let mut new_salt = Bytes::new(&test.env);
    new_salt.append(&test.token_2.address.clone().to_xdr(&test.env)); 
    new_salt.append(&test.token_3.address.clone().to_xdr(&test.env));
    let new_bytes_n_32_salt=test.env.crypto().sha256(&new_salt);
    let new_deterministic_pair_address = test.env.deployer().with_address(test.contract.address.clone(), new_bytes_n_32_salt.clone()).deployed_address();


    let new_expected_new_pair_event: PairEvent = PairEvent {
        token_0: test.token_2.address.clone(),
        token_1: test.token_3.address.clone(),
        pair: new_deterministic_pair_address.clone(),
        new_pairs_length: 2,
    };
    let new_new_pair_event = test.env.events().all().last().unwrap();

    assert_eq!(
        vec![&test.env, new_new_pair_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("RaumFiFactory", Symbol::new(&test.env,"new_pair_created")).into_val(&test.env),
                (new_expected_new_pair_event).into_val(&test.env)
            ),
        ]
    );
}
