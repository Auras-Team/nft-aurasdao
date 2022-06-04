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
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(owner_id.clone(), 5);
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(owner_id.clone())
        .attached_deposit(ONE_NEAR * 22)
        .is_view(false)
        .build());
    contract.nft_mint();
}

/*************/
/* Mint Base */
/*************/

#[test]
fn test_nft_mint() {
    let tkn_a = String::from("token.a");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::ctrl_init_default(acc_x.clone());

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_x.clone(), 1);
    _mint_token(&mut contract, tkn_a.clone(), acc_a.clone(), acc_x.clone());
}

#[test]
#[should_panic(expected = "Invalid deposit, minting cost is 22 near")]
fn test_nft_mint_panic_list_cost() {
    let tkn_a = String::from("token.xyz");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::ctrl_init_default(acc_x.clone());

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
    contract.nft_allow_minting(acc_a.clone(), 1);

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_a.clone())
        .attached_deposit(ONE_NEAR)
        .is_view(false)
        .build());
    contract.nft_mint();
}

#[test]
#[should_panic(expected = "Invalid deposit, minting cost is 30 near")]
fn test_nft_mint_panic_public_cost() {
    let tkn_a = String::from("token.xyz");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::ctrl_init_default(acc_x.clone());

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
    contract.nft_set_mint_info(MintInfo {
        limit: 5,
        public: 30,
        listed: 25,
    });

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_a.clone())
        .attached_deposit(ONE_NEAR)
        .is_view(false)
        .build());
    contract.nft_mint();
}

#[test]
#[should_panic(expected = "Out of tokens to mint")]
fn test_nft_mint_panic_token_id() {
    let tkn_a = String::from("token.a");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::ctrl_init_default(acc_x.clone());

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
    contract.nft_set_mint_info(MintInfo {
        limit: 5,
        public: 26,
        listed: 28,
    });

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_a.clone())
        .attached_deposit(ONE_NEAR * 26)
        .is_view(false)
        .build());
    contract.nft_mint();
    contract.nft_mint();
}

#[test]
#[should_panic(expected = "Account is not authorized to mint")]
fn test_nft_mint_panic_access() {
    let tkn_a = String::from("token.a");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::ctrl_init_default(acc_x.clone());

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
        .predecessor_account_id(acc_a.clone())
        .attached_deposit(ONE_NEAR * 22)
        .is_view(false)
        .build());
    contract.nft_mint();
}

#[test]
#[should_panic(expected = "Must attach 4200000000000000000000 yoctoNEAR to cover storage")]
fn test_nft_register_panic_cost() {
    let tkn_a = String::from("token.a");

    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::ctrl_init_default(acc_x.clone());

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

    let mut contract = Contract::ctrl_init_default(acc_x.clone());

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

/**************/
/* Mint State */
/**************/

#[test]
#[should_panic(expected = "Account has reached minting limit")]
fn test_nft_mint_panic_amount() {
    let tkn_a = String::from("token.a");
    let tkn_b = String::from("token.b");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::ctrl_init_default(acc_x.clone());

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
        .attached_deposit(ONE_NEAR * 22)
        .is_view(false)
        .build());
    contract.nft_mint();
    contract.nft_mint();
}

#[test]
#[should_panic(expected = "Account has reached minting limit")]
fn test_nft_mint_panic_mint_limit() {
    let tkn_a = String::from("token.a");
    let tkn_b = String::from("token.b");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::ctrl_init_default(acc_x.clone());

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
        tkn_a.clone(),
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
    contract.nft_set_mint_info(MintInfo {
        limit: 1,
        public: 30,
        listed: 25,
    });

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_a.clone())
        .attached_deposit(ONE_NEAR * 30)
        .is_view(false)
        .build());
    contract.nft_mint();
    contract.nft_mint();
}

#[test]
#[should_panic(expected = "Account has reached minting limit")]
fn test_nft_mint_panic_mint_disabled() {
    let tkn_a = String::from("token.a");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::ctrl_init_default(acc_x.clone());

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
    contract.nft_register(map);

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_set_mint_info(MintInfo {
        limit: 0,
        public: 30,
        listed: 25,
    });

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_a.clone())
        .attached_deposit(ONE_NEAR * 30)
        .is_view(false)
        .build());
    contract.nft_mint();
}

/************************/
/* Minting Enumeration  */
/************************/

#[test]
fn test_nft_mint_state() {
    let tkn_a = String::from("token.a");
    let tkn_b = String::from("token.b");
    let tkn_c = String::from("token.c");
    let tkn_d = String::from("token.d");
    let tkn_e = String::from("token.e");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_b = AccountId::new_unchecked(String::from("account.b"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::ctrl_init_default(acc_x.clone());

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(REG_COST * 5)
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
    map.insert(
        tkn_c.clone(),
        TokenMetadata {
            title: tkn_c.clone(),
            media: "bb".to_string(),
            media_hash: "cc".to_string(),
            attributes: "dd".to_string(),
        },
    );
    map.insert(
        tkn_d.clone(),
        TokenMetadata {
            title: tkn_d.clone(),
            media: "bb".to_string(),
            media_hash: "cc".to_string(),
            attributes: "dd".to_string(),
        },
    );
    map.insert(
        tkn_e.clone(),
        TokenMetadata {
            title: tkn_e.clone(),
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
    contract.nft_set_mint_info(MintInfo {
        limit: 6,
        public: 24,
        listed: 12,
    });

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_b.clone(), 3);

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_a.clone())
        .is_view(true)
        .build());
    let mut state = contract.nft_mint_state(acc_a.clone(), None, None);
    assert!(state.cost == 24, "unexpected cost");
    assert!(state.count == 0, "unexpected count");
    assert!(state.limit == 6, "unexpected limit");
    assert!(state.tokens.is_empty(), "unexpected tokens");

    state = contract.nft_mint_state(acc_b.clone(), None, None);
    assert!(state.cost == 12, "unexpected cost");
    assert!(state.count == 0, "unexpected count");
    assert!(state.limit == 3, "unexpected limit");
    assert!(state.tokens.is_empty(), "unexpected tokens");

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_a.clone())
        .attached_deposit(ONE_NEAR * 24)
        .is_view(false)
        .build());
    contract.nft_mint();
    contract.nft_mint();

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_b.clone())
        .attached_deposit(ONE_NEAR * 12)
        .is_view(false)
        .build());
    contract.nft_mint();

    state = contract.nft_mint_state(acc_a.clone(), None, None);
    assert!(state.cost == 24, "unexpected cost");
    assert!(state.count == 2, "unexpected count");
    assert!(state.limit == 4, "unexpected limit");
    assert!(state.tokens.len() == 2, "unexpected tokens");

    state = contract.nft_mint_state(acc_b.clone(), None, None);
    assert!(state.cost == 12, "unexpected cost");
    assert!(state.count == 1, "unexpected count");
    assert!(state.limit == 2, "unexpected limit");
    assert!(state.tokens.len() == 1, "unexpected tokens");

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_b.clone())
        .attached_deposit(ONE_NEAR * 12)
        .is_view(false)
        .build());
    contract.nft_mint();
    contract.nft_mint();

    state = contract.nft_mint_state(acc_b.clone(), Some(1), None);
    assert!(state.cost == 12, "unexpected cost");
    assert!(state.count == 3, "unexpected count");
    assert!(state.limit == 0, "unexpected limit");
    assert!(state.tokens.len() == 2, "unexpected tokens");
}
