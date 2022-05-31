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
        .attached_deposit(ONE_NEAR * 22)
        .is_view(false)
        .build());
    contract.nft_mint(token_id.clone(), owner_id.clone());
}

/*************/
/* Mint Base */
/*************/

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
    _mint_token(&mut contract, tkn_a.clone(), acc_a.clone(), acc_x.clone());
}

#[test]
#[should_panic(expected = "Insufishend deposit, minting cost is 22 near")]
fn test_nft_mint_panic_list_cost() {
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
        .attached_deposit(ONE_NEAR)
        .is_view(false)
        .build());
    contract.nft_mint(tkn_a.clone(), acc_a.clone());
}

#[test]
#[should_panic(expected = "Insufishend deposit, minting cost is 30 near")]
fn test_nft_mint_panic_public_cost() {
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
    contract.nft_set_mint_info(MintInfo {
        limit: 5,
        public: 30,
        listed: 25,
    });

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(ONE_NEAR)
        .is_view(false)
        .build());
    contract.nft_mint(tkn_a.clone(), acc_a.clone());
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
    contract.nft_mint(tkn_a.clone(), acc_a.clone());
    contract.nft_mint(tkn_a.clone(), acc_a.clone());
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
    contract.nft_mint(tkn_a.clone(), acc_a.clone());
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
        .attached_deposit(ONE_NEAR * 22)
        .is_view(false)
        .build());
    contract.nft_mint(tkn_a.clone(), acc_a.clone());
}

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
        .attached_deposit(ONE_NEAR * 22)
        .is_view(false)
        .build());
    contract.nft_mint(tkn_a.clone(), acc_a.clone());
    contract.nft_mint(tkn_b.clone(), acc_a.clone());
}

#[test]
#[should_panic(expected = "Account has reached minting limit")]
fn test_nft_mint_panic_mint_limit() {
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
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(ONE_NEAR * 30)
        .is_view(false)
        .build());
    contract.nft_mint(tkn_a.clone(), acc_a.clone());
    contract.nft_mint(tkn_b.clone(), acc_a.clone());
}

#[test]
#[should_panic(expected = "Account has reached minting limit")]
fn test_nft_mint_panic_mint_disabled() {
    let tkn_a = String::from("token.a");

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
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(ONE_NEAR * 30)
        .is_view(false)
        .build());
    contract.nft_mint(tkn_a.clone(), acc_a.clone());
}
