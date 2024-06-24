use crate::test::{RaumFiPairTest}; 
use soroban_sdk::{String};
use crate::test::pair::PairError;

#[test]
fn initialize_initial_values_0() {
    let test = RaumFiPairTest::setup();
    assert_eq!(test.factory.fee_to(), test.admin);
    assert_eq!(test.factory.fee_to_setter(), test.admin);
    assert_eq!(test.factory.fees_enabled(), false);
    
    assert_eq!(test.token_0.symbol(), String::from_str(&test.env, "TOK0"));
    assert_eq!(test.token_1.symbol(), String::from_str(&test.env, "ABCDEFGHIJ"));
    assert_eq!(test.token_0.name(), String::from_str(&test.env, "Token 0"));
    assert_eq!(test.token_1.name(), String::from_str(&test.env, "Token 1"));

    // Test liqpool initial values:
    test.contract.initialize(&test.factory.address, &test.token_0.address, &test.token_1.address);
    assert_eq!(test.contract.token_0(), test.token_0.address);
    assert_eq!(test.contract.token_1(), test.token_1.address);
    assert_eq!(test.contract.factory(), test.factory.address);
    assert_eq!(test.contract.get_reserves(), (0,0));
    assert_eq!(test.contract.k_multiplier(), 0);
    assert_eq!(test.contract.total_supply(), 0);
    assert_eq!(test.contract.k_multiplier(), 0);
    
    assert_eq!(test.contract.symbol(), String::from_str(&test.env, "TOK0-ABCDEF-RAUM-LP"));
    assert_eq!(test.contract.name(), String::from_str(&test.env, "TOK0-ABCDEF RAUM LP Token"));
    assert_eq!(test.contract.decimals(), 7);
}
