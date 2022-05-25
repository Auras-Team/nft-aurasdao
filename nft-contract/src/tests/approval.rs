use super::*;

use crate::approval::NftApproval;

fn _mint_token(
    contract: &mut Contract,
    token_id: String,
    owner_id: AccountId,
    creator_id: AccountId,
) {
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
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(creator_id.clone())
        .attached_deposit(REG_COST)
        .is_view(false)
        .build());
    contract.nft_register(map);

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(creator_id.clone())
        .attached_deposit(MINT_COST)
        .is_view(false)
        .build());
    contract.nft_mint(token_id.clone(), owner_id.clone(), None);
}

#[test]
fn test_nft_approval() {
    let tkn_a = String::from("token.a");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_b = AccountId::new_unchecked(String::from("account.b"));
    let acc_c = AccountId::new_unchecked(String::from("account.c"));
    let acc_d = AccountId::new_unchecked(String::from("account.d"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init_default(acc_x.clone());

    // Approve interface
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_x.clone(), 1);
    _mint_token(&mut contract, tkn_a.clone(), acc_a.clone(), acc_x.clone());

    // testing_env!(VMContextBuilder::new()
    //     .predecessor_account_id(acc_x.clone())
    //     .attached_deposit(MINT_COST)
    //     .is_view(false)
    //     .build());
    // contract.nft_mint(
    //     tkn_a.clone(),
    //     TokenMetadata {
    //         title: tkn_a.clone(),
    //         media: "bb".to_string(),
    //         media_hash: "cc".to_string(),
    //         attributes: "dd".to_string(),
    //         issued_at: 1,
    //     },
    //     acc_a.clone(),
    //     None,
    // );

    testing_env!(VMContextBuilder::new().is_view(true).build());
    let token = contract.nft_token(tkn_a.clone()).expect("must be set");
    assert!(token.owner_id == acc_a);

    // Approve interface
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_a.clone())
        .attached_deposit(210000000000000000000)
        .is_view(false)
        .build());

    contract.nft_approve(tkn_a.clone(), acc_b.clone(), None);
    contract.nft_approve(tkn_a.clone(), acc_c.clone(), None);
    contract.nft_approve(tkn_a.clone(), acc_d.clone(), None);

    // Is Approved interface
    testing_env!(VMContextBuilder::new()
        .attached_deposit(0)
        .is_view(true)
        .build());

    assert!(contract.nft_is_approved(tkn_a.clone(), acc_b.clone(), None));
    assert!(contract.nft_is_approved(tkn_a.clone(), acc_b.clone(), Some(0)));
    assert!(contract.nft_is_approved(tkn_a.clone(), acc_c.clone(), None));
    assert!(contract.nft_is_approved(tkn_a.clone(), acc_c.clone(), Some(1)));
    assert!(contract.nft_is_approved(tkn_a.clone(), acc_d.clone(), None));
    assert!(contract.nft_is_approved(tkn_a.clone(), acc_d.clone(), Some(2)));

    // Revoke/All interface
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_a.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());

    contract.nft_revoke(tkn_a.clone(), acc_b.clone());

    assert!(!contract.nft_is_approved(tkn_a.clone(), acc_b.clone(), None));
    assert!(!contract.nft_is_approved(tkn_a.clone(), acc_b.clone(), Some(0)));
    assert!(contract.nft_is_approved(tkn_a.clone(), acc_c.clone(), None));
    assert!(contract.nft_is_approved(tkn_a.clone(), acc_c.clone(), Some(1)));
    assert!(contract.nft_is_approved(tkn_a.clone(), acc_d.clone(), None));
    assert!(contract.nft_is_approved(tkn_a.clone(), acc_d.clone(), Some(2)));

    contract.nft_revoke_all(tkn_a.clone());

    assert!(!contract.nft_is_approved(tkn_a.clone(), acc_b.clone(), None));
    assert!(!contract.nft_is_approved(tkn_a.clone(), acc_b.clone(), Some(0)));
    assert!(!contract.nft_is_approved(tkn_a.clone(), acc_c.clone(), None));
    assert!(!contract.nft_is_approved(tkn_a.clone(), acc_c.clone(), Some(1)));
    assert!(!contract.nft_is_approved(tkn_a.clone(), acc_d.clone(), None));
    assert!(!contract.nft_is_approved(tkn_a.clone(), acc_d.clone(), Some(2)));

    // Approved transfer interface
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_a.clone())
        .attached_deposit(210000000000000000000)
        .is_view(false)
        .build());

    contract.nft_approve(tkn_a.clone(), acc_b.clone(), None);

    assert!(contract.nft_is_approved(tkn_a.clone(), acc_b.clone(), None));
    assert!(contract.nft_is_approved(tkn_a.clone(), acc_b.clone(), Some(3)));
    assert!(!contract.nft_is_approved(tkn_a.clone(), acc_c.clone(), None));
    assert!(!contract.nft_is_approved(tkn_a.clone(), acc_c.clone(), Some(1)));
    assert!(!contract.nft_is_approved(tkn_a.clone(), acc_d.clone(), None));
    assert!(!contract.nft_is_approved(tkn_a.clone(), acc_d.clone(), Some(2)));

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_b.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_transfer(acc_c.clone(), tkn_a.clone(), Some(3), None);

    assert!(!contract.nft_is_approved(tkn_a.clone(), acc_b.clone(), None));
    assert!(!contract.nft_is_approved(tkn_a.clone(), acc_b.clone(), Some(3)));

    // Verify the transfer
    testing_env!(VMContextBuilder::new().is_view(true).build());
    let token = contract.nft_token(tkn_a.clone()).expect("must be set");
    assert!(token.owner_id == acc_c.clone())
}

/***************/
/* Token Yocto */
/***************/

#[test]
#[should_panic(expected = "Requires attached deposit of at least 1 yoctoNEAR")]
fn test_nft_xxx_panic_yocto() {
    let tkn_a = String::from("token.a");
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init_default(acc_x.clone());

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(0)
        .is_view(false)
        .build());
    contract.nft_approve(tkn_a.clone(), acc_x.clone(), None);
}

#[test]
#[should_panic(expected = "Requires attached deposit of exactly 1 yoctoNEAR")]
fn test_nft_revoke_panic_yocto() {
    let tkn_a = String::from("token.a");
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init_default(acc_x.clone());

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(0)
        .is_view(false)
        .build());
    contract.nft_revoke(tkn_a.clone(), acc_x.clone());
}

#[test]
#[should_panic(expected = "Requires attached deposit of exactly 1 yoctoNEAR")]
fn test_nft_revoke_all_panic_yocto() {
    let tkn_a = String::from("token.a");
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init_default(acc_x.clone());

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(0)
        .is_view(false)
        .build());
    contract.nft_revoke_all(tkn_a.clone());
}

/*******************/
/* Token Not Found */
/*******************/

#[test]
#[should_panic(expected = "Token not found")]
fn test_nft_approve_panic_not_found() {
    let tkn_a = String::from("token.a");
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init_default(acc_x.clone());

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_approve(tkn_a.clone(), acc_x.clone(), None);
}

#[test]
#[should_panic(expected = "Token not found")]
fn test_nft_revoke_panic_not_found() {
    let tkn_a = String::from("token.a");
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init_default(acc_x.clone());

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_revoke(tkn_a.clone(), acc_x.clone());
}

#[test]
#[should_panic(expected = "Token not found")]
fn test_nft_revoke_all_panic_not_found() {
    let tkn_a = String::from("token.a");
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init_default(acc_x.clone());

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_revoke_all(tkn_a.clone());
}

/*********************/
/* Access Owner Only */
/*********************/

#[test]
#[should_panic(expected = "Only owner can approve transfer access")]
fn test_nft_approve_panic_access() {
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

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(210000000000000000000)
        .is_view(false)
        .build());
    contract.nft_approve(tkn_a.clone(), acc_x.clone(), None);
}

#[test]
#[should_panic(expected = "Only owner can revoke transfer access")]
fn test_nft_revoke_panic_access() {
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

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_revoke(tkn_a.clone(), acc_x.clone());
}

#[test]
#[should_panic(expected = "Only owner can revoke all transfer access")]
fn test_nft_revoke_all_panic_access() {
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

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_revoke_all(tkn_a.clone());
}

/******************************/
/* Only approved can transfer */
/******************************/

#[test]
#[should_panic(expected = "Unauthorized transfer")]
fn test_nft_approval_panic_transfer() {
    let tkn_a = String::from("token.a");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_b = AccountId::new_unchecked(String::from("account.b"));
    let acc_c = AccountId::new_unchecked(String::from("account.c"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init_default(acc_x.clone());

    // Approve interface
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_x.clone(), 1);
    _mint_token(&mut contract, tkn_a.clone(), acc_a.clone(), acc_x.clone());

    testing_env!(VMContextBuilder::new().is_view(true).build());
    let token = contract.nft_token(tkn_a.clone()).expect("must be set");
    assert!(token.owner_id == acc_a);

    // Approve check
    testing_env!(VMContextBuilder::new().is_view(true).build());

    assert!(!contract.nft_is_approved(tkn_a.clone(), acc_b.clone(), None));
    assert!(!contract.nft_is_approved(tkn_a.clone(), acc_b.clone(), Some(3)));

    // Approve transfer
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_b.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_transfer(acc_c.clone(), tkn_a.clone(), Some(3), None);
}

#[test]
#[should_panic(expected = "The actual approval_id 0 is different from the given approval_id 1")]
fn test_nft_approval_panic_transfer_id() {
    let tkn_a = String::from("token.a");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_b = AccountId::new_unchecked(String::from("account.b"));
    let acc_c = AccountId::new_unchecked(String::from("account.c"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init_default(acc_x.clone());

    // Approve interface
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_x.clone(), 1);
    _mint_token(&mut contract, tkn_a.clone(), acc_a.clone(), acc_x.clone());

    testing_env!(VMContextBuilder::new().is_view(true).build());
    let token = contract.nft_token(tkn_a.clone()).expect("must be set");
    assert!(token.owner_id == acc_a);

    // Approve setup
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_a.clone())
        .attached_deposit(MINT_COST)
        .is_view(false)
        .build());

    contract.nft_approve(tkn_a.clone(), acc_b.clone(), None);

    // Approve check
    testing_env!(VMContextBuilder::new().is_view(true).build());

    assert!(contract.nft_is_approved(tkn_a.clone(), acc_b.clone(), None));
    assert!(!contract.nft_is_approved(tkn_a.clone(), acc_b.clone(), Some(1)));

    // Approve transfer
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_b.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_transfer(acc_c.clone(), tkn_a.clone(), Some(1), None);
}
