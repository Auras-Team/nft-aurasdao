use super::*;

use crate::metadata::NftMetadata;

#[test]
fn test_nft_metadata_contract() {
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let contract = Contract::nft_init(acc_x.clone());
    let data = contract.nft_metadata();

    // Note: this values are hard coded in to Contract::new

    assert!(data.spec == "nft-2.0.0".to_string());
    assert!(data.name == "Auras".to_string());
    assert!(data.symbol == "AURA".to_string());

    assert!(data.icon.is_none());
    assert!(data.base_uri.is_none());
    assert!(data.reference.is_none());
    assert!(data.reference_hash.is_none());
}

#[test]
fn test_nft_metadata_token() {
    let tkn_a = String::from("token.a");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init(acc_x.clone());

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_x.clone());

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
        .attached_deposit(7000000000000000000000)
        .is_view(false)
        .build());
    contract.nft_mint(tkn_a.clone(), metadata, acc_a.clone(), None);

    testing_env!(VMContextBuilder::new().is_view(true).build());
    let data = contract.nft_token(tkn_a.clone()).expect("nust be set");

    assert!(data.metadata.title.expect("nust be set") == tkn_a.clone());
    assert!(data.metadata.description.expect("nust be set") == "aa".to_string());
    assert!(data.metadata.media.expect("nust be set") == "bb".to_string());
    assert!(data.metadata.media_hash.expect("nust be set") == "cc".to_string());

    assert!(data.metadata.copies.expect("nust be set") == 0);
    assert!(data.metadata.issued_at.expect("nust be set") == 1);
    assert!(data.metadata.expires_at.expect("nust be set") == 2);
    assert!(data.metadata.starts_at.expect("nust be set") == 3);
    assert!(data.metadata.updated_at.expect("nust be set") == 4);

    assert!(data.metadata.extra.expect("nust be set") == "dd".to_string());
    assert!(data.metadata.reference.expect("nust be set") == "ee".to_string());
    assert!(data.metadata.reference_hash.expect("nust be set") == "ff".to_string());
}
