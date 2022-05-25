use super::*;

use crate::royalty::NftRoyalties;

#[test]
fn test_nft_royalty() {
    let tkn_a = String::from("token.a");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_b = AccountId::new_unchecked(String::from("account.b"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init_default(acc_x.clone());

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(REG_COST)
        .is_view(false)
        .build());
    let mut map = HashMap::new();
    map.insert(
        tkn_a.clone(),
        TokenMetadata {
            title: tkn_a.clone(),
            media: "bb".to_string(),
            media_hash: "cc".to_string(),
            attributes: "dd".to_string(),
        },
    );
    contract.nft_register(map);
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_x.clone(), 1);

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(8000000000000000000000)
        .is_view(false)
        .build());
    contract.nft_mint(tkn_a.clone(), acc_a.clone());

    // Check the token info
    testing_env!(VMContextBuilder::new().is_view(true).build());
    let token = contract.nft_token(tkn_a.clone()).expect("must be set");

    assert!(token.royalty.get(&acc_x.clone()).expect("must be set") == &(600));

    //increase balance to adjust computed numbers
    let royalty = contract.nft_payout(tkn_a.clone(), U128::from(100000), 32);

    assert!(royalty.payout.get(&acc_x.clone()).expect("must be set") == &U128::from(6000));
    assert!(royalty.payout.get(&acc_a.clone()).expect("must be set") == &U128::from(94000));

    // Check trasfer results
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_a.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    let transfer = contract.nft_transfer_payout(
        acc_b.clone(),
        tkn_a.clone(),
        None,
        None,
        U128::from(1000),
        32,
    );

    assert!(transfer.payout.get(&acc_a.clone()).expect("must be set") == &U128::from(940));
    assert!(transfer.payout.get(&acc_x.clone()).expect("must be set") == &U128::from(60));

    // Check the token info
    testing_env!(VMContextBuilder::new().is_view(true).build());
    let token = contract.nft_token(tkn_a.clone()).expect("must be set");

    assert!(token.owner_id == acc_b.clone());
    assert!(token.royalty.get(&acc_x.clone()).expect("must be set") == &(600));
}

#[test]
#[should_panic(expected = "Token not found")]
fn test_nft_payouts_panic_token() {
    let tkn_a = String::from("token.a");

    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let contract = Contract::nft_init_default(acc_x.clone());

    // Check the token info
    testing_env!(VMContextBuilder::new().is_view(true).build());

    //increase balance to adjust computed numbers
    contract.nft_payout(tkn_a.clone(), U128::from(100000), 32);
}

#[test]
#[should_panic(expected = "Market cannot payout royalties")]
fn test_nft_payouts_panic_count() {
    let tkn_a = String::from("token.a");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init_default(acc_x.clone());

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(REG_COST)
        .is_view(false)
        .build());
    let mut map = HashMap::new();
    map.insert(
        tkn_a.clone(),
        TokenMetadata {
            title: tkn_a.clone(),
            media: "bb".to_string(),
            media_hash: "cc".to_string(),
            attributes: "dd".to_string(),
        },
    );
    contract.nft_register(map);
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_x.clone(), 1);

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(8000000000000000000000)
        .is_view(false)
        .build());
    contract.nft_mint(tkn_a.clone(), acc_a.clone());

    // Check the token info
    testing_env!(VMContextBuilder::new().is_view(true).build());
    let token = contract.nft_token(tkn_a.clone()).expect("must be set");

    assert!(token.royalty.get(&acc_x.clone()).expect("must be set") == &(600));

    //increase balance to adjust computed numbers
    contract.nft_payout(tkn_a.clone(), U128::from(100000), 0);
}

#[test]
#[should_panic(expected = "Market cannot payout royalties")]
fn test_nft_transfer_payouts_panic_count() {
    let tkn_a = String::from("token.a");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_b = AccountId::new_unchecked(String::from("account.b"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init_default(acc_x.clone());

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(REG_COST)
        .is_view(false)
        .build());
    let mut map = HashMap::new();
    map.insert(
        tkn_a.clone(),
        TokenMetadata {
            title: tkn_a.clone(),
            media: "bb".to_string(),
            media_hash: "cc".to_string(),
            attributes: "dd".to_string(),
        },
    );
    contract.nft_register(map);
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_x.clone(), 1);

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(8000000000000000000000)
        .is_view(false)
        .build());
    contract.nft_mint(tkn_a.clone(), acc_a.clone());

    // Check the token info
    testing_env!(VMContextBuilder::new().is_view(true).build());
    let token = contract.nft_token(tkn_a.clone()).expect("must be set");

    assert!(token.royalty.get(&acc_x.clone()).expect("must be set") == &(600));

    // Check trasfer results
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_a.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_transfer_payout(
        acc_b.clone(),
        tkn_a.clone(),
        None,
        None,
        U128::from(1000),
        0,
    );
}
