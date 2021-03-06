use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, UnorderedMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, require, AccountId, Balance, CryptoHash, PanicOnDefault, Promise,
    PromiseOrValue, ONE_NEAR,
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

    //cost of minting a token
    pub mint_info: LazyOption<MintInfo>,

    //keep track of accounts and amount that can be minted
    pub mint_state_list: UnorderedMap<AccountId, MintState>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: UnorderedMap<TokenId, Token>,

    //keeps track of the token metadata for a given token ID
    pub meta_data_by_id: UnorderedMap<TokenId, TokenMetadata>,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: UnorderedMap<AccountId, UnorderedSet<TokenId>>,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    ContractMetadata,
    ContractMintState,
    MintStateList,

    TokensById,
    MetaDataById,

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
    pub fn ctrl_init_default(owner_id: AccountId) -> Self {
        Self::ctrl_init(
            owner_id,
            MintInfo {
                limit: 5,
                public: 0,
                listed: 22,
            },
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
    pub fn ctrl_init(owner_id: AccountId, info: MintInfo, metadata: ContractMetadata) -> Self {
        // Initialize data and return it
        Self {
            //Set the contract data fields equal to the passed in owner_id.
            owner_id,
            metadata: LazyOption::new(
                StorageKey::ContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
            mint_info: LazyOption::new(
                StorageKey::ContractMintState.try_to_vec().unwrap(),
                Some(&info),
            ),

            tokens_by_id: UnorderedMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            tokens_per_owner: UnorderedMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),

            meta_data_by_id: UnorderedMap::new(StorageKey::MetaDataById.try_to_vec().unwrap()),
            mint_state_list: UnorderedMap::new(StorageKey::MintStateList.try_to_vec().unwrap()),
        }
    }
}

/*******************/
/* Near Withdrawal */
/*******************/

#[near_bindgen]
impl Contract {
    pub fn ctrl_owner(&self) -> AccountId {
        self.owner_id.clone()
    }

    pub fn ctrl_storage_cost(&self) -> U128 {
        U128::from(u128::from(env::storage_usage()) * env::storage_byte_cost())
    }

    pub fn ctrl_storage_usage(&self) -> u64 {
        env::storage_usage()
    }

    #[payable]
    pub fn ctrl_withdrawal(&mut self, amount: U128) {
        //require that the owner attached 1 yoctoNEAR for security reasons
        require_one_yocto();
        //require the the sender is the owner of the contract
        require!(
            env::predecessor_account_id() == self.owner_id,
            "Only owner can withdrawal funds",
        );
        Promise::new(env::predecessor_account_id()).transfer(u128::from(amount));
    }
}

/*******************************/
/* Danger Contract Destruction */
/*             ====            */
/*   only use for development  */
/*******************************/

// #[near_bindgen]
// impl Contract {
//     pub fn dev_clear_tokens(&mut self) {
//         let list: Vec<String> = self.meta_data_by_id.keys().take(10).collect();
//         require!(list.len() > 0, "all tokens have been removed");

//         for key in list {
//             if let Some(token) = self.tokens_by_id.get(&key.clone()) {
//                 self.mint_state_list.remove(&token.owner_id);
//                 self.tokens_per_owner.remove(&token.owner_id);
//             }
//             self.tokens_by_id.remove(&key.clone());
//             self.meta_data_by_id.remove(&key.clone());
//         }
//     }

//     pub fn dev_clear_owners(&mut self) {
//         let list: Vec<AccountId> = self.tokens_per_owner.keys().take(50).collect();
//         require!(list.len() > 0, "all owners have been removed");

//         for key in list {
//             self.tokens_per_owner.remove(&key);
//         }
//     }

//     pub fn dev_clear_mint_state(&mut self) {
//         let list: Vec<AccountId> = self.mint_state_list.keys().take(50).collect();
//         require!(list.len() > 0, "all mint info has been removed");

//         for key in list {
//             self.mint_state_list.remove(&key);
//         }
//     }
// }

/*****************/
/* Unit Testing  */
/*****************/

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;
