use crate::test::{RaumFiPairTest};
use crate::test::deposit::add_liquidity;
use num_integer::Roots; 



#[test]
fn fee_off() {
    let test = RaumFiPairTest::setup();    
    test.env.budget().reset_unlimited();
    test.contract.initialize(&test.factory.address, &test.token_0.address, &test.token_1.address);
    let amount_0: i128 = 50_000_000;
    let amount_1: i128 = 100_000_000;
    let expected_liquidity: i128 =  70_710_678;
    let minimum_liquidity: i128 = 1_00;

    assert_eq!(test.contract.k_multiplier(), 0);
    add_liquidity(&test, &amount_0, &amount_1);
    assert_eq!(test.contract.get_reserves(), (amount_0,amount_1,));
    assert_eq!(test.contract.k_multiplier(), 0);

    let swap_amount_0 = 10_000_000;
    let expected_output_amount_1 = 16624979;

    test.token_0.transfer(&test.user, &test.contract.address, &swap_amount_0);
    test.contract.swap(&0, &expected_output_amount_1, &test.user);
    assert_eq!(test.contract.get_reserves(), (amount_0+swap_amount_0,amount_1-expected_output_amount_1,));
    assert_eq!(test.contract.k_multiplier(), 0);

    test.contract.transfer(&test.user, &test.contract.address, &expected_liquidity.checked_sub(minimum_liquidity).unwrap());

    test.contract.withdraw(&test.user);
    assert_eq!(test.contract.k_multiplier(), 0);
    assert_eq!(test.contract.balance(&test.user), 0);
        assert_eq!(test.contract.total_supply(), minimum_liquidity);
        assert_eq!(test.contract.balance(&test.contract.address), minimum_liquidity);
        assert_eq!(test.token_0.balance(&test.contract.address), 85);
        assert_eq!(test.token_1.balance(&test.contract.address), 118);
        assert_eq!(test.contract.get_reserves(), (85,118));

}

// Testing fee when doing add_liquiquidity/swap/add_liquidity
#[test]
fn fee_on_add_swap_add() {
    let test = RaumFiPairTest::setup();    
    test.env.budget().reset_unlimited();
    test.factory.set_fees_enabled(&true);
    assert_eq!(test.factory.fees_enabled(), true);
    assert_eq!(test.factory.fee_to(), test.admin);
    test.contract.initialize(&test.factory.address, &test.token_0.address, &test.token_1.address);

    let amount_0: i128 = 50_000_000;
    let amount_1: i128 = 100_000_000;
    let minimum_liquidity: i128 = 1_00;
    let expected_liquidity: i128 =  70_710_678;
    let original_0: i128 = test.token_0.balance(&test.user);
    let original_1: i128 = test.token_1.balance(&test.user);

    // ***************** DEPOSIT *****************
    assert_eq!(test.contract.k_multiplier(), 0);
    add_liquidity(&test, &amount_0, &amount_1);

    // If we deposit with fee on, we should see a change in the klast paramenter
    //klast should be the new reserves (amount0 and amount1)
    assert_eq!(test.contract.k_multiplier(), amount_0.checked_mul(amount_1).unwrap());
    assert_eq!(test.contract.total_supply(), expected_liquidity);
    assert_eq!(test.token_0.balance(&test.user), original_0.checked_sub(amount_0).unwrap());
    assert_eq!(test.token_1.balance(&test.user), original_1.checked_sub(amount_1).unwrap());

    // ***************** SWAP *****************

    let swap_amount_0 = 10_000_000;
    // Amount does not changes... only the fee is splitted differently
    let expected_output_amount_1 = 16624979;

    test.token_0.transfer(&test.user, &test.contract.address, &swap_amount_0);
    test.contract.swap(&0, &expected_output_amount_1, &test.user);
    //klast does not gets updated in swaps
    assert_eq!(test.contract.k_multiplier(), amount_0.checked_mul(amount_1).unwrap());
    
    let new_expected_reserve_0= 60000000; //amount_0+swap_amount_0
    assert_eq!(new_expected_reserve_0, amount_0+swap_amount_0);

    let new_expected_reserve_1= 83375021; // amount_1-expected_output_amount_1; // 83375021
    assert_eq!(new_expected_reserve_1, amount_1-expected_output_amount_1);

    assert_eq!(test.contract.get_reserves(), (new_expected_reserve_0,new_expected_reserve_1));
    assert_eq!(test.token_0.balance(&test.user), original_0-amount_0-swap_amount_0);
    assert_eq!(test.token_1.balance(&test.user), original_1-amount_1+expected_output_amount_1);
    
    let k2_root=70728362; // new_expected_reserve_0.checked_mul(new_expected_reserve_1).unwrap().sqrt();
    assert_eq!(new_expected_reserve_0.checked_mul(new_expected_reserve_1).unwrap().sqrt(), k2_root);

    let k1_root = 70_710_678; // amount_0.checked_mul(amount_1).unwrap().sqrt();
    assert_eq!(amount_0.checked_mul(amount_1).unwrap().sqrt(), k1_root);

    // After the swap, k2 should be greater than k1
    assert_eq!(k2_root > k1_root, true);



    // ***************** DEPOSIT AGAIN! *****************
    assert_eq!(test.contract.total_supply(), expected_liquidity);
    assert_eq!(test.contract.get_reserves(), (new_expected_reserve_0,new_expected_reserve_1));
    let new_amount_0: i128 = 1_000_000;
    let new_amount_1: i128 = 1389583; //(new_amount_0*new_expected_reserve_1)/new_expected_reserve_0);
    assert_eq!(new_amount_1, (new_amount_0*new_expected_reserve_1)/new_expected_reserve_0);
    let new_got_liquidity = add_liquidity(&test, &new_amount_0, &new_amount_1);
    assert_eq!(test.token_0.balance(&test.user), original_0-amount_0-swap_amount_0-new_amount_0);
    assert_eq!(test.token_1.balance(&test.user), original_1-amount_1+expected_output_amount_1-new_amount_1);

    assert_eq!(test.contract.k_multiplier(), (new_expected_reserve_0+new_amount_0).checked_mul(new_expected_reserve_1+new_amount_1).unwrap());
    
    
    let n = 2946;
    let numerator = expected_liquidity.checked_mul(k2_root-k1_root).unwrap();
    assert_eq!(numerator, 1250447629752);
    let denominator = (5_i128).checked_mul(k2_root).unwrap().checked_add(k1_root).unwrap();
    assert_eq!(denominator, 424352488);
    assert_eq!(n, numerator/denominator);
    assert_eq!(numerator.checked_div(denominator).unwrap(), n);

    
    let shares_0 = 1178560; //new_amount_0 * (expected_liquidity+n) / new_expected_reserve_0;
    assert_eq!(shares_0, (new_amount_0 * (expected_liquidity+n)) / new_expected_reserve_0);


    
    let shares_1 = 1178559;
    assert_eq!(shares_1, (new_amount_1 * (expected_liquidity+n)) / new_expected_reserve_1);

    let expected_minted_liquidity = 1178559;
    assert_eq!(new_got_liquidity, expected_minted_liquidity);


    
    // whe should have minted n shares to the admin:
    assert_eq!(test.contract.total_supply(), expected_liquidity+n+expected_minted_liquidity);
    assert_eq!(test.contract.balance(&test.contract.address), minimum_liquidity);
    assert_eq!(test.contract.balance(&test.admin), n);
    assert_eq!(test.contract.balance(&test.user), expected_minted_liquidity+ (expected_liquidity-minimum_liquidity));


}
