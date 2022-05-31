use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
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

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<TokenId, Token>,

    //keep track of accounts and amount that can be minted
    pub mint_state_list: LookupMap<AccountId, MintState>,

    //keeps track of the token metadata for a given token ID
    pub token_data_by_id: UnorderedMap<TokenId, TokenMetadata>,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    ContractMetadata,
    ContractMintState,
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
            //Storage keys are simply the prefixes used for storage to avoid data collision.
            mint_state_list: LookupMap::new(
                StorageKey::ContractAllowListMint.try_to_vec().unwrap(),
            ),

            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_data_by_id: UnorderedMap::new(StorageKey::TokenDataById.try_to_vec().unwrap()),

            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
        }
    }
}

/******************/
/* Minting State */
/******************/

#[near_bindgen]
impl Contract {
    pub fn nft_mint_info(&self) -> MintInfo {
        self.mint_info.get().unwrap()
    }

    #[payable]
    pub fn nft_set_mint_info(&mut self, info: MintInfo) {
        //require that the owner attached 1 yoctoNEAR for security reasons
        require_one_yocto();
        //require the the sender is the owner of the contract
        require!(
            env::predecessor_account_id() == self.owner_id,
            "Only owner can set mint info",
        );
        self.mint_info.set(&info);
    }
}

/*********************/
/* Minting Whitelist */
/*********************/

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_allow_minting(&mut self, account_id: AccountId, amount: u32) {
        //require that the owner attached 1 yoctoNEAR for security reasons
        require_one_yocto();
        //require the the sender is the owner of the contract
        require!(
            env::predecessor_account_id() == self.owner_id,
            "Only owner can allow minting access",
        );
        //insert the account and the limit to the minting whitelist
        self.mint_state_list.insert(
            &account_id,
            &MintState {
                limit: amount,
                listed: true,
            },
        );
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
        self.mint_state_list.remove(&account_id);
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

/*****************/
/* Unit Testing  */
/*****************/

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;
