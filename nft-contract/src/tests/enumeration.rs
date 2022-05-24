use super::*;

use crate::enumeration::NftEnumeration;

fn _mint_token(
    contract: &mut Contract,
    token_id: String,
    owner_id: AccountId,
    creator_id: AccountId,
) {
    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(creator_id.clone())
        .attached_deposit(6000000000000000000000)
        .is_view(false)
        .build());
    contract.nft_mint(
        token_id.clone(),
        TokenMetadata {
            title: Some(token_id.clone()),
            description: None,
            media: None,
            media_hash: None,
            copies: None,
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        },
        owner_id.clone(),
        None,
    );
}

#[test]
fn test_nft_enumeration() {
    let tkn_a = String::from("token.a");
    let tkn_b = String::from("token.b");
    let tkn_c = String::from("token.c");
    let tkn_d = String::from("token.d");
    let tkn_e = String::from("token.e");

    let acc_a = AccountId::new_unchecked(String::from("account.a"));
    let acc_b = AccountId::new_unchecked(String::from("account.b"));
    let acc_c = AccountId::new_unchecked(String::from("account.c"));
    let acc_x = AccountId::new_unchecked(String::from("account.x"));

    let mut contract = Contract::nft_init_default(acc_x.clone());

    testing_env!(VMContextBuilder::new()
        .predecessor_account_id(acc_x.clone())
        .attached_deposit(1)
        .is_view(false)
        .build());
    contract.nft_allow_minting(acc_x.clone(), 5);

    _mint_token(&mut contract, tkn_a.clone(), acc_a.clone(), acc_x.clone());
    _mint_token(&mut contract, tkn_b.clone(), acc_b.clone(), acc_x.clone());

    testing_env!(VMContextBuilder::new().is_view(true).build());
    assert!(contract.nft_total_supply() == U128::from(2));
    assert!(contract.nft_supply_for_owner(acc_a.clone()) == U128::from(1));
    assert!(contract.nft_supply_for_owner(acc_b.clone()) == U128::from(1));
    assert!(contract.nft_supply_for_owner(acc_c.clone()) == U128::from(0));

    _mint_token(&mut contract, tkn_c.clone(), acc_a.clone(), acc_x.clone());
    _mint_token(&mut contract, tkn_d.clone(), acc_b.clone(), acc_x.clone());
    _mint_token(&mut contract, tkn_e.clone(), acc_b.clone(), acc_x.clone());

    testing_env!(VMContextBuilder::new().is_view(true).build());
    assert!(contract.nft_total_supply() == U128::from(5));
    assert!(contract.nft_supply_for_owner(acc_a.clone()) == U128::from(2));
    assert!(contract.nft_supply_for_owner(acc_b.clone()) == U128::from(3));
    assert!(contract.nft_supply_for_owner(acc_c.clone()) == U128::from(0));

    let map_x = _token_lsit_to_map(contract.nft_tokens(None, None));
    assert!(_token_lsit_to_map(contract.nft_tokens(Some(U128::from(2)), Some(2))).len() == 2);
    assert!(_token_lsit_to_map(contract.nft_tokens(Some(U128::from(2)), None)).len() == 3);
    assert!(map_x.len() == 5);

    assert!(map_x.get(&tkn_a).is_some());
    assert!(map_x.get(&tkn_b).is_some());
    assert!(map_x.get(&tkn_c).is_some());
    assert!(map_x.get(&tkn_d).is_some());
    assert!(map_x.get(&tkn_e).is_some());

    let map_a = _token_lsit_to_map(contract.nft_tokens_for_owner(acc_a.clone(), None, None));
    let map_aa = _token_lsit_to_map(contract.nft_tokens_for_owner(
        acc_a.clone(),
        Some(U128::from(1)),
        Some(1),
    ));
    assert!(map_aa.len() == 1);
    assert!(map_a.len() == 2);

    assert!(map_a.get(&tkn_a).is_some());
    assert!(map_a.get(&tkn_c).is_some());

    let map_b = _token_lsit_to_map(contract.nft_tokens_for_owner(acc_b.clone(), None, None));
    let map_bb = _token_lsit_to_map(contract.nft_tokens_for_owner(
        acc_b.clone(),
        Some(U128::from(1)),
        Some(2),
    ));
    assert!(map_bb.len() == 2);
    assert!(map_b.len() == 3);

    assert!(map_b.get(&tkn_b).is_some());
    assert!(map_b.get(&tkn_d).is_some());
    assert!(map_b.get(&tkn_e).is_some());
}
