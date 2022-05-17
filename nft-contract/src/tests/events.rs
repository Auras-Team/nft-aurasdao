use super::*;

#[test]
fn nep_format_vector() {
    let expected = r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"nft_mint","data":[{"owner_id":"foundation.near","token_ids":["aurora","proximitylabs"]},{"owner_id":"user1.near","token_ids":["meme"]}]}"#;
    let log = EventLog {
        standard: "nep171".to_string(),
        version: "1.0.0".to_string(),
        event: EventLogVariant::NftMint(vec![
            NftMintLog {
                owner_id: "foundation.near".to_owned(),
                token_ids: vec!["aurora".to_string(), "proximitylabs".to_string()],
                memo: None,
            },
            NftMintLog {
                owner_id: "user1.near".to_owned(),
                token_ids: vec!["meme".to_string()],
                memo: None,
            },
        ]),
    };
    assert_eq!(expected, log.to_string());
}

#[test]
fn nep_format_mint() {
    let expected = r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"nft_mint","data":[{"owner_id":"foundation.near","token_ids":["aurora","proximitylabs"]}]}"#;
    let log = EventLog {
        standard: "nep171".to_string(),
        version: "1.0.0".to_string(),
        event: EventLogVariant::NftMint(vec![NftMintLog {
            owner_id: "foundation.near".to_owned(),
            token_ids: vec!["aurora".to_string(), "proximitylabs".to_string()],
            memo: None,
        }]),
    };
    assert_eq!(expected, log.to_string());
}

#[test]
fn nep_format_transfer_all_fields() {
    let expected = r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"nft_transfer","data":[{"authorized_id":"market.near","old_owner_id":"user1.near","new_owner_id":"user2.near","token_ids":["token"],"memo":"Go Team!"}]}"#;
    let log = EventLog {
        standard: "nep171".to_string(),
        version: "1.0.0".to_string(),
        event: EventLogVariant::NftTransfer(vec![NftTransferLog {
            authorized_id: Some("market.near".to_string()),
            old_owner_id: "user1.near".to_string(),
            new_owner_id: "user2.near".to_string(),
            token_ids: vec!["token".to_string()],
            memo: Some("Go Team!".to_owned()),
        }]),
    };
    assert_eq!(expected, log.to_string());
}
