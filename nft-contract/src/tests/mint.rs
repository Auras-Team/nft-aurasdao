use super::*;

fn _mint_token(
    contract: &mut Contract,
    token_id: String,
    owner_id: AccountId,
    creator_id: AccountId,
) {
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(creator_id.clone())
        .attached_deposit(REG_COST)
        .is_view(false)
        .build());
    let mut map = HashMap::new();
    map.insert(
        token_id.clone(),
        TokenMetadata {
            title: token_id.clone(),
            media: "bb".to_string(),
            media_hash: "cc".to_string(),
            attributes: "dd".to_string(),
        },
    );
    contract.nft_register(map);
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(creator_id.clone())
        .attached_deposit(MINT_COST)
        .is_view(false)
        .build());
    contract.nft_mint(token_id.clone(), owner_id.clone(), None);
}

#[test]
fn test_nft_mint() {
    let tkn_a = String::from("token.a");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init_default(acc_x.clone());

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_x.clone(), 1);

    // let mut royalties = HashMap::new();
    // royalties.insert(AccountId::new_unchecked(String::from("royalty.a")), 1250);
    // royalties.insert(AccountId::new_unchecked(String::from("royalty.b")), 1250);
    // royalties.insert(AccountId::new_unchecked(String::from("royalty.c")), 1250);
    // royalties.insert(AccountId::new_unchecked(String::from("royalty.d")), 1250);
    // royalties.insert(AccountId::new_unchecked(String::from("royalty.e")), 2500);
    // royalties.insert(AccountId::new_unchecked(String::from("royalty.f")), 2500);

    // let metadata = TokenMetadata {
    //     title: tkn_a.clone(),
    //     description: "aa".to_string(),
    //     media: "bb".to_string(),
    //     media_hash: "cc".to_string(),
    //     extra: "dd".to_string(),
    //     issued_at: 1,
    // };

    // testing_env!(VMContextBuilder::new()
    //     .predecessor_account_id(acc_x.clone())
    //     .attached_deposit(8000000000000000000000)
    //     .is_view(false)
    //     .build());
    // contract.nft_mint(tkn_a.clone(), metadata, acc_a.clone(), Some(royalties));
    _mint_token(&mut contract, tkn_a.clone(), acc_a.clone(), acc_x.clone());
}

#[test]
#[should_panic(expected = "Must attach 4190000000000000000000 yoctoNEAR to cover storage")]
fn test_nft_mint_panic_cost() {
    let tkn_a = String::from("token.xyz");

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
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_mint(tkn_a.clone(), acc_a.clone(), None);
}

#[test]
#[should_panic(expected = "Token id already minted")]
fn test_nft_mint_panic_token_id() {
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
    contract.nft_allow_minting(acc_a.clone(), 2);

    // let mut royalties = HashMap::new();
    // royalties.insert(AccountId::new_unchecked(String::from("royalty.a")), 100);
    // royalties.insert(AccountId::new_unchecked(String::from("royalty.b")), 100);
    // royalties.insert(AccountId::new_unchecked(String::from("royalty.c")), 100);
    // royalties.insert(AccountId::new_unchecked(String::from("royalty.d")), 100);
    // royalties.insert(AccountId::new_unchecked(String::from("royalty.e")), 100);
    // royalties.insert(AccountId::new_unchecked(String::from("royalty.f")), 100);

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_a.clone())
        .attached_deposit(MINT_COST)
        .is_view(false)
        .build());
    contract.nft_mint(tkn_a.clone(), acc_a.clone(), None);
    contract.nft_mint(tkn_a.clone(), acc_a.clone(), None);
}

#[test]
#[should_panic(expected = "Account is not authorized to mint")]
fn test_nft_mint_panic_access() {
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

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_x.clone(), 1);

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_a.clone())
        .attached_deposit(7000000000000000000000)
        .is_view(false)
        .build());
    contract.nft_mint(tkn_a.clone(), acc_a.clone(), None);
}

#[test]
#[should_panic(expected = "Token id could not be found")]
fn test_nft_mint_panic_id_not_found() {
    let tkn_a = String::from("token.a");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init_default(acc_x.clone());

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_a.clone(), 1);

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_a.clone())
        .attached_deposit(7000000000000000000000)
        .is_view(false)
        .build());
    contract.nft_mint(tkn_a.clone(), acc_a.clone(), None);
}

#[test]
#[should_panic(expected = "Account has no mints remaining")]
fn test_nft_mint_panic_amount() {
    let tkn_a = String::from("token.a");
    let tkn_b = String::from("token.b");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init_default(acc_x.clone());

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(REG_COST * 2)
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
    map.insert(
        tkn_b.clone(),
        TokenMetadata {
            title: tkn_b.clone(),
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
    contract.nft_allow_minting(acc_a.clone(), 1);

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_a.clone())
        .attached_deposit(7000000000000000000000)
        .is_view(false)
        .build());
    contract.nft_mint(tkn_a.clone(), acc_a.clone(), None);
    contract.nft_mint(tkn_b.clone(), acc_a.clone(), None);
}

// #[test]
// #[should_panic(expected = "Cannot add more than 6 perpetual royalty amounts")]
// fn test_nft_mint_panic_royalty_count() {
//     let tkn_a = String::from("token.a");

//     let acc_a = AccountId::new_unchecked(String::from("account.a"));
//     let acc_x = AccountId::new_unchecked(String::from("account.x"));

//     let mut contract = Contract::nft_init_default(acc_x.clone());

//     testing_env!(VMContextBuilder::new()
//         .predecessor_account_id(acc_x.clone())
//         .attached_deposit(1)
//         .is_view(false)
//         .build());
//     contract.nft_allow_minting(acc_x.clone(), 1);

//     let mut royalties = HashMap::new();
//     royalties.insert(AccountId::new_unchecked(String::from("royalty.a")), 100);
//     royalties.insert(AccountId::new_unchecked(String::from("royalty.b")), 100);
//     royalties.insert(AccountId::new_unchecked(String::from("royalty.c")), 100);
//     royalties.insert(AccountId::new_unchecked(String::from("royalty.d")), 100);
//     royalties.insert(AccountId::new_unchecked(String::from("royalty.e")), 100);
//     royalties.insert(AccountId::new_unchecked(String::from("royalty.f")), 100);
//     royalties.insert(AccountId::new_unchecked(String::from("royalty.x")), 100);

//     let metadata = TokenMetadata {
//         title: tkn_a.clone(),
//         description: "aa".to_string(),
//         media: "bb".to_string(),
//         media_hash: "cc".to_string(),
//         extra: "dd".to_string(),
//         issued_at: 1,
//     };

//     testing_env!(VMContextBuilder::new()
//         .predecessor_account_id(acc_x.clone())
//         .attached_deposit(8000000000000000000000)
//         .is_view(false)
//         .build());
//     contract.nft_mint(tkn_a.clone(), metadata, acc_a.clone(), Some(royalties));
// }

// #[test]
// #[should_panic(expected = "Total royalty shares can not be more then 10000 base points")]
// fn test_nft_mint_panic_royalty_amount() {
//     let tkn_a = String::from("token.a");

//     let acc_a = AccountId::new_unchecked(String::from("account.a"));
//     let acc_x = AccountId::new_unchecked(String::from("account.x"));

//     let mut contract = Contract::nft_init_default(acc_x.clone());

//     testing_env!(VMContextBuilder::new()
//         .predecessor_account_id(acc_x.clone())
//         .attached_deposit(1)
//         .is_view(false)
//         .build());
//     contract.nft_allow_minting(acc_x.clone(), 1);

//     let mut royalties = HashMap::new();
//     royalties.insert(AccountId::new_unchecked(String::from("royalty.a")), 2500);
//     royalties.insert(AccountId::new_unchecked(String::from("royalty.b")), 2500);
//     royalties.insert(AccountId::new_unchecked(String::from("royalty.c")), 2500);
//     royalties.insert(AccountId::new_unchecked(String::from("royalty.d")), 2500);
//     royalties.insert(AccountId::new_unchecked(String::from("royalty.e")), 1);

//     let metadata = TokenMetadata {
//         title: tkn_a.clone(),
//         description: "aa".to_string(),
//         media: "bb".to_string(),
//         media_hash: "cc".to_string(),
//         extra: "dd".to_string(),
//         issued_at: 1,
//     };

//     testing_env!(VMContextBuilder::new()
//         .predecessor_account_id(acc_x.clone())
//         .attached_deposit(8000000000000000000000)
//         .is_view(false)
//         .build());
//     contract.nft_mint(tkn_a.clone(), metadata, acc_a.clone(), Some(royalties));
// }

#[test]
#[should_panic(expected = "Must attach 4200000000000000000000 yoctoNEAR to cover storage")]
fn test_nft_register_panic_cost() {
    let tkn_a = String::from("token.a");

    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init_default(acc_x.clone());

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    let mut map = HashMap::new();
    map.insert(
        tkn_a.clone(),
        TokenMetadata {
            title: tkn_a.clone(),
            media: "012345678901234567890123456789012".to_string(),
            media_hash: "0123456789012345678901234567890123456789012345678901234567890123".to_string(),
            attributes: "01234567890123456789012345678901234567890123456789012345678901230123456789012345678901234567890123456789012345678901234567890123".to_string(),
        },
    );
    contract.nft_register(map);
}

#[test]
#[should_panic(expected = "Token id is already registered")]
fn test_nft_register_panic_id_used() {
    let tkn_a = String::from("token.a");

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
    contract.nft_register(map.clone());
    contract.nft_register(map.clone());
}
