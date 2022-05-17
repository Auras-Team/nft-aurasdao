use super::*;

use crate::royalty::NftRoyalties;

#[test]
fn test_nft_royalty() {
    let tkn_a = String::from("token.a");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_b = AccountId::new_unchecked(String::from("account.b"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let acr_a = AccountId::new_unchecked(String::from("royalty.a"));
    let acr_b = AccountId::new_unchecked(String::from("royalty.b"));
    let acr_c = AccountId::new_unchecked(String::from("royalty.c"));
    let acr_d = AccountId::new_unchecked(String::from("royalty.d"));
    let acr_e = AccountId::new_unchecked(String::from("royalty.e"));
    let acr_f = AccountId::new_unchecked(String::from("royalty.f"));

    let mut contract = Contract::nft_init(acc_x.clone());

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_x.clone());

    let mut royalties = HashMap::new();
    royalties.insert(acr_a.clone(), 100);
    royalties.insert(acr_b.clone(), 200);
    royalties.insert(acr_c.clone(), 300);
    royalties.insert(acr_d.clone(), 400);
    royalties.insert(acr_e.clone(), 500);
    royalties.insert(acr_f.clone(), 600);

    let metadata = TokenMetadata {
        title: Some(tkn_a.clone()),
        description: Some("aa".to_string()),
        media: Some("bb".to_string()),
        media_hash: Some("cc".to_string()),
        copies: Some(0),
        issued_at: Some(1),
        expires_at: Some(2),
        starts_at: Some(3),
        updated_at: Some(4),
        extra: Some("dd".to_string()),
        reference: Some("ee".to_string()),
        reference_hash: Some("ff".to_string()),
    };

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(8000000000000000000000)
        .is_view(false)
        .build());
    contract.nft_mint(tkn_a.clone(), metadata, acc_a.clone(), Some(royalties));

    // Check the token info
    testing_env!(VMContextBuilder::new().is_view(true).build());
    let token = contract.nft_token(tkn_a.clone()).expect("must be set");

    assert!(token.royalty.get(&acr_a.clone()).expect("must be set") == &(100));
    assert!(token.royalty.get(&acr_b.clone()).expect("must be set") == &(200));
    assert!(token.royalty.get(&acr_c.clone()).expect("must be set") == &(300));
    assert!(token.royalty.get(&acr_d.clone()).expect("must be set") == &(400));
    assert!(token.royalty.get(&acr_e.clone()).expect("must be set") == &(500));
    assert!(token.royalty.get(&acr_f.clone()).expect("must be set") == &(600));

    //increase balance to adjust computed numbers
    let royalty = contract.nft_payout(tkn_a.clone(), U128::from(100000), 32);

    assert!(royalty.payout.get(&acr_a.clone()).expect("must be set") == &U128::from(1000));
    assert!(royalty.payout.get(&acr_b.clone()).expect("must be set") == &U128::from(2000));
    assert!(royalty.payout.get(&acr_c.clone()).expect("must be set") == &U128::from(3000));
    assert!(royalty.payout.get(&acr_d.clone()).expect("must be set") == &U128::from(4000));
    assert!(royalty.payout.get(&acr_e.clone()).expect("must be set") == &U128::from(5000));
    assert!(royalty.payout.get(&acr_f.clone()).expect("must be set") == &U128::from(6000));
    assert!(royalty.payout.get(&acc_a.clone()).expect("must be set") == &U128::from(79000));

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

    assert!(transfer.payout.get(&acr_a.clone()).expect("must be set") == &U128::from(10));
    assert!(transfer.payout.get(&acr_b.clone()).expect("must be set") == &U128::from(20));
    assert!(transfer.payout.get(&acr_c.clone()).expect("must be set") == &U128::from(30));
    assert!(transfer.payout.get(&acr_d.clone()).expect("must be set") == &U128::from(40));
    assert!(transfer.payout.get(&acr_e.clone()).expect("must be set") == &U128::from(50));
    assert!(transfer.payout.get(&acr_f.clone()).expect("must be set") == &U128::from(60));
    assert!(transfer.payout.get(&acc_a.clone()).expect("must be set") == &U128::from(790));

    // Check the token info
    testing_env!(VMContextBuilder::new().is_view(true).build());
    let token = contract.nft_token(tkn_a.clone()).expect("must be set");

    assert!(token.owner_id == acc_b.clone());
    assert!(token.royalty.get(&acr_a.clone()).expect("must be set") == &(100));
    assert!(token.royalty.get(&acr_b.clone()).expect("must be set") == &(200));
    assert!(token.royalty.get(&acr_c.clone()).expect("must be set") == &(300));
    assert!(token.royalty.get(&acr_d.clone()).expect("must be set") == &(400));
    assert!(token.royalty.get(&acr_e.clone()).expect("must be set") == &(500));
    assert!(token.royalty.get(&acr_f.clone()).expect("must be set") == &(600));
}

#[test]
#[should_panic(expected = "Token not found")]
fn test_nft_payouts_panic_token() {
    let tkn_a = String::from("token.a");

    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init(acc_x.clone());

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_x.clone());

    // Check the token info
    testing_env!(VMContextBuilder::new().is_view(true).build());

    //increase balance to adjust computed numbers
    contract.nft_payout(tkn_a.clone(), U128::from(100000), 32);
}

#[test]
#[should_panic(expected = "Market cannot payout to that many receivers")]
fn test_nft_payouts_panic_count() {
    let tkn_a = String::from("token.a");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let acr_a = AccountId::new_unchecked(String::from("royalty.a"));
    let acr_b = AccountId::new_unchecked(String::from("royalty.b"));
    let acr_c = AccountId::new_unchecked(String::from("royalty.c"));
    let acr_d = AccountId::new_unchecked(String::from("royalty.d"));
    let acr_e = AccountId::new_unchecked(String::from("royalty.e"));
    let acr_f = AccountId::new_unchecked(String::from("royalty.f"));

    let mut contract = Contract::nft_init(acc_x.clone());

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_x.clone());

    let mut royalties = HashMap::new();
    royalties.insert(acr_a.clone(), 100);
    royalties.insert(acr_b.clone(), 200);
    royalties.insert(acr_c.clone(), 300);
    royalties.insert(acr_d.clone(), 400);
    royalties.insert(acr_e.clone(), 500);
    royalties.insert(acr_f.clone(), 600);

    let metadata = TokenMetadata {
        title: Some(tkn_a.clone()),
        description: Some("aa".to_string()),
        media: Some("bb".to_string()),
        media_hash: Some("cc".to_string()),
        copies: Some(0),
        issued_at: Some(1),
        expires_at: Some(2),
        starts_at: Some(3),
        updated_at: Some(4),
        extra: Some("dd".to_string()),
        reference: Some("ee".to_string()),
        reference_hash: Some("ff".to_string()),
    };

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(8000000000000000000000)
        .is_view(false)
        .build());
    contract.nft_mint(tkn_a.clone(), metadata, acc_a.clone(), Some(royalties));

    // Check the token info
    testing_env!(VMContextBuilder::new().is_view(true).build());

    //increase balance to adjust computed numbers
    contract.nft_payout(tkn_a.clone(), U128::from(100000), 4);
}

#[test]
#[should_panic(expected = "Market cannot payout to that many receivers")]
fn test_nft_transfer_payouts_panic_count() {
    let tkn_a = String::from("token.a");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_b = AccountId::new_unchecked(String::from("account.b"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let acr_a = AccountId::new_unchecked(String::from("royalty.a"));
    let acr_b = AccountId::new_unchecked(String::from("royalty.b"));
    let acr_c = AccountId::new_unchecked(String::from("royalty.c"));
    let acr_d = AccountId::new_unchecked(String::from("royalty.d"));
    let acr_e = AccountId::new_unchecked(String::from("royalty.e"));
    let acr_f = AccountId::new_unchecked(String::from("royalty.f"));

    let mut contract = Contract::nft_init(acc_x.clone());

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_x.clone());

    let mut royalties = HashMap::new();
    royalties.insert(acr_a.clone(), 100);
    royalties.insert(acr_b.clone(), 200);
    royalties.insert(acr_c.clone(), 300);
    royalties.insert(acr_d.clone(), 400);
    royalties.insert(acr_e.clone(), 500);
    royalties.insert(acr_f.clone(), 600);

    let metadata = TokenMetadata {
        title: Some(tkn_a.clone()),
        description: Some("aa".to_string()),
        media: Some("bb".to_string()),
        media_hash: Some("cc".to_string()),
        copies: Some(0),
        issued_at: Some(1),
        expires_at: Some(2),
        starts_at: Some(3),
        updated_at: Some(4),
        extra: Some("dd".to_string()),
        reference: Some("ee".to_string()),
        reference_hash: Some("ff".to_string()),
    };

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(8000000000000000000000)
        .is_view(false)
        .build());
    contract.nft_mint(tkn_a.clone(), metadata, acc_a.clone(), Some(royalties));

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
        4,
    );
}
