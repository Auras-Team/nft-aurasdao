use super::*;

use crate::metadata::NftMetadata;

#[test]
fn test_nft_metadata_contract() {
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mintstate = MintInfo {
        limit: 5,
        public: 0,
        listed: 42,
    };

    let metadata = ContractMetadata {
        spec: "nft-1.0.0".to_string(),
        name: "AA".to_string(),
        symbol: "BB".to_string(),
        icon: Some("CC".to_string()),
        base_uri: Some("DD".to_string()),
        reference: Some("EE".to_string()),
        reference_hash: Some("FF".to_string()),
    };

    let contract = Contract::ctrl_init(acc_x.clone(), mintstate.clone(), metadata.clone());
    let data = contract.nft_metadata();

    // Note: this values are hard coded in to Contract::new

    assert!(data.spec == metadata.spec);
    assert!(data.name == metadata.name);
    assert!(data.symbol == metadata.symbol);

    assert!(data.icon.expect("must be set") == metadata.icon.unwrap());
    assert!(data.base_uri.expect("must be set") == metadata.base_uri.unwrap());
    assert!(data.reference.expect("must be set") == metadata.reference.unwrap());
    assert!(data.reference_hash.expect("must be set") == metadata.reference_hash.unwrap());
}

#[test]
fn test_nft_metadata_contract_default() {
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let contract = Contract::ctrl_init_default(acc_x.clone());
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
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(ONE_NEAR * 42)
        .is_view(false)
        .build());
    contract.nft_mint(tkn_a.clone(), acc_a.clone());

    testing_env!(VMContextBuilder::new().is_view(true).build());
    let data = contract.nft_token(tkn_a.clone()).expect("nust be set");

    assert!(data.metadata.title.expect("nust be set") == tkn_a.clone());
    assert!(data.metadata.description.is_some());
    assert!(data.metadata.media.expect("nust be set") == "bb".to_string());
    assert!(data.metadata.media_hash.expect("nust be set") == "cc".to_string());

    assert!(data.metadata.copies.expect("nust be set") == 1);
    assert!(data.metadata.issued_at.is_some());
    assert!(data.metadata.updated_at.is_some());
    assert!(data.metadata.expires_at.is_none());
    assert!(data.metadata.starts_at.is_none());

    assert!(data.metadata.extra.expect("nust be set") == "dd".to_string());
    assert!(data.metadata.reference.is_none());
    assert!(data.metadata.reference_hash.is_none());
}
