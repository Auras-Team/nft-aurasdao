use super::*;

#[test]
fn test_nft_mint() {
    let tkn_a = String::from("token.a");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init(acc_x.clone());

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_x.clone(), 1);

    let mut royalties = HashMap::new();
    royalties.insert(AccountId::new_unchecked(String::from("royalty.a")), 1250);
    royalties.insert(AccountId::new_unchecked(String::from("royalty.b")), 1250);
    royalties.insert(AccountId::new_unchecked(String::from("royalty.c")), 1250);
    royalties.insert(AccountId::new_unchecked(String::from("royalty.d")), 1250);
    royalties.insert(AccountId::new_unchecked(String::from("royalty.e")), 2500);
    royalties.insert(AccountId::new_unchecked(String::from("royalty.f")), 2500);

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
}

#[test]
#[should_panic(expected = "Must attach 7780000000000000000000 yoctoNEAR to cover storage")]
fn test_nft_mint_panic_cost() {
    let tkn_a = String::from("token.a");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init(acc_x.clone());

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_x.clone(), 1);

    let mut royalties = HashMap::new();
    royalties.insert(AccountId::new_unchecked(String::from("royalty.a")), 100);
    royalties.insert(AccountId::new_unchecked(String::from("royalty.b")), 100);
    royalties.insert(AccountId::new_unchecked(String::from("royalty.c")), 100);
    royalties.insert(AccountId::new_unchecked(String::from("royalty.d")), 100);
    royalties.insert(AccountId::new_unchecked(String::from("royalty.e")), 100);
    royalties.insert(AccountId::new_unchecked(String::from("royalty.f")), 100);

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
        .attached_deposit(100)
        .is_view(false)
        .build());
    contract.nft_mint(tkn_a.clone(), metadata, acc_a.clone(), Some(royalties));
}

#[test]
#[should_panic(expected = "Token id already exists")]
fn test_nft_mint_panic_token_id() {
    let tkn_a = String::from("token.a");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init(acc_x.clone());

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_x.clone(), 2);

    let mut royalties = HashMap::new();
    royalties.insert(AccountId::new_unchecked(String::from("royalty.a")), 100);
    royalties.insert(AccountId::new_unchecked(String::from("royalty.b")), 100);
    royalties.insert(AccountId::new_unchecked(String::from("royalty.c")), 100);
    royalties.insert(AccountId::new_unchecked(String::from("royalty.d")), 100);
    royalties.insert(AccountId::new_unchecked(String::from("royalty.e")), 100);
    royalties.insert(AccountId::new_unchecked(String::from("royalty.f")), 100);

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(8000000000000000000000)
        .is_view(false)
        .build());
    contract.nft_mint(
        tkn_a.clone(),
        TokenMetadata {
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
        },
        acc_a.clone(),
        Some(royalties.clone()),
    );
    contract.nft_mint(
        tkn_a.clone(),
        TokenMetadata {
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
        },
        acc_a.clone(),
        Some(royalties),
    );
}

#[test]
#[should_panic(expected = "Account is not authorized to mint")]
fn test_nft_mint_panic_access() {
    let tkn_a = String::from("token.a");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init(acc_x.clone());

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_x.clone(), 1);

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
        .predecessor_account_id(acc_a.clone())
        .attached_deposit(7000000000000000000000)
        .is_view(false)
        .build());
    contract.nft_mint(tkn_a.clone(), metadata, acc_a.clone(), None);
}

#[test]
#[should_panic(expected = "Account has no mints remaining")]
fn test_nft_mint_panic_amount() {
    let tkn_a = String::from("token.a");
    let tkn_b = String::from("token.b");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init(acc_x.clone());

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_a.clone(), 1);

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
        .predecessor_account_id(acc_a.clone())
        .attached_deposit(7000000000000000000000)
        .is_view(false)
        .build());
    contract.nft_mint(tkn_a.clone(), metadata.clone(), acc_a.clone(), None);
    contract.nft_mint(tkn_b.clone(), metadata.clone(), acc_a.clone(), None);
}

#[test]
#[should_panic(expected = "Cannot add more than 6 perpetual royalty amounts")]
fn test_nft_mint_panic_royalty_count() {
    let tkn_a = String::from("token.a");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init(acc_x.clone());

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_x.clone(), 1);

    let mut royalties = HashMap::new();
    royalties.insert(AccountId::new_unchecked(String::from("royalty.a")), 100);
    royalties.insert(AccountId::new_unchecked(String::from("royalty.b")), 100);
    royalties.insert(AccountId::new_unchecked(String::from("royalty.c")), 100);
    royalties.insert(AccountId::new_unchecked(String::from("royalty.d")), 100);
    royalties.insert(AccountId::new_unchecked(String::from("royalty.e")), 100);
    royalties.insert(AccountId::new_unchecked(String::from("royalty.f")), 100);
    royalties.insert(AccountId::new_unchecked(String::from("royalty.x")), 100);

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
}

#[test]
#[should_panic(expected = "Total royalty shares can not be more then 10000 base points")]
fn test_nft_mint_panic_royalty_amount() {
    let tkn_a = String::from("token.a");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init(acc_x.clone());

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_x.clone(), 1);

    let mut royalties = HashMap::new();
    royalties.insert(AccountId::new_unchecked(String::from("royalty.a")), 2500);
    royalties.insert(AccountId::new_unchecked(String::from("royalty.b")), 2500);
    royalties.insert(AccountId::new_unchecked(String::from("royalty.c")), 2500);
    royalties.insert(AccountId::new_unchecked(String::from("royalty.d")), 2500);
    royalties.insert(AccountId::new_unchecked(String::from("royalty.e")), 1);

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
}

// TODO Check limit error
