use near_sdk::test_utils::VMContextBuilder;
use near_sdk::{testing_env, AccountId};

use crate::*;

const REG_COST: Balance = 4200000000000000000000;
const MINT_COST: Balance = 6200000000000000000000;

mod approval;
mod enumeration;
mod events;
mod metadata;
mod mint;
mod royalty;

#[test]
fn test_nft_approval_allow_access() {
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

    // Test nft_allow_minting: pass
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_a.clone(), 1);

    // Test nft_allow_minting: check
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_a.clone())
        .attached_deposit(MINT_COST)
        .is_view(false)
        .build());
    contract.nft_mint(tkn_a.clone(), acc_b.clone());

    testing_env!(VMContextBuilder::new().is_view(true).build());
    let token = contract.nft_token(tkn_a.clone()).expect("must be set");
    assert!(token.owner_id == acc_b);
}

/***************/
/* Token Yocto */
/***************/

#[test]
#[should_panic(expected = "Requires attached deposit of exactly 1 yoctoNEAR")]
fn test_nft_allow_minting_panic_yocto() {
    let acc_b = AccountId::new_unchecked(String::from("account.b"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init_default(acc_x.clone());

    // Test nft_allow_minting: access error
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(0)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_b.clone(), 1);
}

#[test]
#[should_panic(expected = "Requires attached deposit of exactly 1 yoctoNEAR")]
fn test_nft_revoke_minting_panic_yocto() {
    let acc_b = AccountId::new_unchecked(String::from("account.b"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init_default(acc_x.clone());

    // Test nft_revoke_minting: access error
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(0)
        .is_view(false)
        .build());
    contract.nft_revoke_minting(acc_b.clone());
}

/*********************/
/* Access Owner Only */
/*********************/

#[test]
#[should_panic(expected = "Only owner can allow minting access")]
fn test_nft_allow_minting_panic_access() {
    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_b = AccountId::new_unchecked(String::from("account.b"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init_default(acc_x.clone());

    // Test nft_allow_minting: access error
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_a.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_b.clone(), 1);
}

#[test]
#[should_panic(expected = "Only owner can revoke minting access")]
fn test_nft_revoke_minting_panic_access() {
    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_b = AccountId::new_unchecked(String::from("account.b"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init_default(acc_x.clone());

    // Test nft_revoke_minting: access error
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_a.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_revoke_minting(acc_b.clone());
}

/******************/
/* Test Transfer */
/******************/

#[test]
#[should_panic(expected = "Token not found")]
fn test_nft_transfer_panic_token() {
    let tkn_a = String::from("token.a");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init_default(acc_x.clone());

    // Approve interface
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_x.clone(), 1);

    // Approve transfer
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_a.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_transfer(acc_a.clone(), tkn_a.clone(), Some(1), None);
}

#[test]
#[should_panic(expected = "The token owner and the receiver should be different")]
fn test_nft_transfer_panic_owner() {
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

    // Approve interface
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_x.clone(), 1);

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(MINT_COST)
        .is_view(false)
        .build());
    contract.nft_mint(tkn_a.clone(), acc_a.clone());

    testing_env!(VMContextBuilder::new().is_view(true).build());
    let token = contract.nft_token(tkn_a.clone()).expect("must be set");
    assert!(token.owner_id == acc_a);

    // Approve transfer
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_a.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_transfer(acc_a.clone(), tkn_a.clone(), Some(1), None);
}

/******************/
/* Test Utilities */
/******************/

pub(crate) fn _token_lsit_to_map(list: Vec<JsonToken>) -> HashMap<String, JsonToken> {
    let mut map = HashMap::new();
    for token in list {
        map.insert(token.token_id.clone(), token);
    }
    map
}
