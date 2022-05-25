use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, require, AccountId, Balance, CryptoHash, PanicOnDefault, Promise,
    PromiseOrValue,
};
use std::collections::HashMap;

pub use crate::approval::*;
pub use crate::events::*;
use crate::internal::*;
pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::nft_core::*;
pub use crate::royalty::*;

mod approval;
mod enumeration;
mod events;
mod internal;
mod metadata;
mod mint;
mod nft_core;
mod royalty;

/// This is the version of the standard implementation
pub const NFT_STANDARD_SPEC: &str = "1.0.0";
/// This is the name of the NFT standard we're using
pub const NFT_STANDARD_NAME: &str = "nep171";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of the metadata for the contract
    pub metadata: LazyOption<ContractMetadata>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<TokenId, Token>,

    //keep track of accounts and amount that can be minted
    pub allowed_list_mint: LookupMap<AccountId, u64>,

    //keeps track of the token metadata for a given token ID
    pub token_data_by_id: UnorderedMap<TokenId, TokenMetadata>,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    ContractMetadata,
    ContractAllowListMint,

    TokensById,
    TokenDataById,

    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
}

/**************/
/* Initialize */
/**************/

#[near_bindgen]
impl Contract {
    /*
        initialization function (this can only be called once).
        this initializes the contract with defaults and owner_id.
    */
    #[init]
    pub fn nft_init_default(owner_id: AccountId) -> Self {
        Self::nft_init(
            owner_id,
            ContractMetadata {
                spec: "nft-2.0.0".to_string(),
                name: "Auras".to_string(),
                symbol: "AURA".to_string(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    /*
        initialization function (this can only be called once).
        this initializes the contract with metadata and owner_id.
    */
    #[init]
    pub fn nft_init(owner_id: AccountId, metadata: ContractMetadata) -> Self {
        // Initialize data and return it
        Self {
            //Set the contract data fields equal to the passed in owner_id.
            owner_id,
            metadata: LazyOption::new(
                StorageKey::ContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
            //Storage keys are simply the prefixes used for storage to avoid data collision.
            allowed_list_mint: LookupMap::new(
                StorageKey::ContractAllowListMint.try_to_vec().unwrap(),
            ),

            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_data_by_id: UnorderedMap::new(StorageKey::TokenDataById.try_to_vec().unwrap()),

            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
        }
    }
}

/*********************/
/* Minting Whitelist */
/*********************/

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_allow_minting(&mut self, account_id: AccountId, amount: u64) {
        //require that the owner attached 1 yoctoNEAR for security reasons
        require_one_yocto();
        //require the the sender is the owner of the contract
        require!(
            env::predecessor_account_id() == self.owner_id,
            "Only owner can allow minting access",
        );
        //insert the account and the limit to the minting whitelist
        self.allowed_list_mint.insert(&account_id, &amount);
    }

    #[payable]
    pub fn nft_revoke_minting(&mut self, account_id: AccountId) {
        //require that the owner attached 1 yoctoNEAR for security reasons
        require_one_yocto();
        //require the the sender is the owner of the contract
        require!(
            env::predecessor_account_id() == self.owner_id,
            "Only owner can revoke minting access",
        );
        //remove the account to the minting whitelist
        self.allowed_list_mint.remove(&account_id);
    }
}

/*****************/
/* Unit Testing  */
/*****************/

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;
