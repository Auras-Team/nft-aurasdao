use near_sdk::require;

use crate::*;

/***********/
/* Minting */
/***********/

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_register(&mut self, token_list: HashMap<String, TokenMetadata>) {
        //storage so we need at least one yocto
        require_at_least_one_yocto();

        //require the the sender is the owner of the contract
        require!(
            env::predecessor_account_id() == self.owner_id,
            "Only owner is allow to register tokens",
        );
        //enforce the token supply cap
        require!(
            token_list.len() + (self.token_data_by_id.len() as usize) <= 1000,
            "Max supply of 1000 tokens reached",
        );

        //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        for (token_id, metadata) in &token_list {
            require!(
                self.token_data_by_id.insert(token_id, &metadata).is_none(),
                "Token id is already registered"
            );
        }

        //refund any excess storage if the owner attached too much. Panic when short.
        refund_deposit(env::storage_usage() - initial_storage_usage);
    }

    #[payable]
    pub fn nft_mint(&mut self) -> JsonMintState {
        //storage so we need at least one yocto
        require_at_least_one_yocto();

        let sender_id = env::predecessor_account_id();

        //ensure that the predecessor can mint tokens
        let mint_info = self.mint_info.get().expect("Mint info not found");

        // if public price > get account limit or create
        // if no public price > get whitelist limit or error
        let mint_state = match mint_info.public {
            0 => self
                .mint_state_list
                .get(&sender_id)
                .expect("Account is not authorized to mint"),
            _ => self.mint_state_list.get(&sender_id).unwrap_or(MintState {
                limit: mint_info.limit,
                listed: false,
            }),
        };

        // If the account reachedes zero it is no longer allowed to mint
        require!(mint_state.limit > 0, "Account has reached minting limit");

        // Verify atached deposit is amount needed to mint
        match mint_state.listed {
            true => require!(
                env::attached_deposit() >= ONE_NEAR * u128::from(mint_info.listed),
                format!("Invalid deposit, minting cost is {} near", mint_info.listed)
            ),
            false => require!(
                env::attached_deposit() >= ONE_NEAR * u128::from(mint_info.public),
                format!("Invalid deposit, minting cost is {} near", mint_info.public)
            ),
        }

        require!(
            self.tokens_by_id.len() < self.token_data_by_id.len(),
            "Out of tokens to mint"
        );

        let list: Vec<String> = self
            .token_data_by_id
            .keys()
            .skip(self.tokens_by_id.len() as usize)
            .take(1)
            .collect();

        let token_id = list.get(0).expect("Unable to find token data");

        //specify the token struct that contains the owner ID
        let token = Token {
            owner_id: sender_id.clone(),
            approved_account_ids: Default::default(),
            next_approval_id: 0,
            issued_at: env::block_timestamp(),
        };
        //insert the token ID and token struct and make sure that it was not minted before.
        require!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token id already minted"
        );

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &token_id);

        // Construct the mint log as per the events standard.
        let nft_mint_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_STANDARD_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftMint(vec![NftMintLog {
                // Owner of the token.
                owner_id: token.owner_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_mint_log.to_string());

        //update the mint counter for the senders account
        self.mint_state_list.insert(
            &sender_id,
            &MintState {
                limit: mint_state.limit - 1,
                listed: mint_state.listed,
            },
        );

        let tokens = self
            .tokens_per_owner
            .get(&sender_id)
            .expect("Token lsit not found");

        JsonMintState {
            cost: match mint_state.listed {
                true => mint_info.listed,
                false => mint_info.public,
            },
            count: tokens.len(),
            limit: mint_state.limit,
            tokens: tokens
                .iter()
                //skip to the index we specified in the start variable
                .skip(0 as usize)
                //take the first "limit" elements in the vector. If we didn't specify a limit, use 10
                .take(10 as usize)
                //we'll map the token IDs which are strings into Json Tokens
                .map(|token_id| self.nft_token(token_id.clone()).unwrap())
                //since we turned the keys into an iterator, we need to turn it back into a vector to return
                .collect(),
        }
    }
}

/****************/
/* Minting Info */
/****************/

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

/************************/
/* Minting Enumeration  */
/************************/

#[near_bindgen]
impl Contract {
    pub fn nft_mint_state(
        &self,
        account_id: AccountId,
        from_index: Option<u64>,
        limit: Option<u64>,
    ) -> JsonMintState {
        //needed for prices and to see if public minting is enabled
        let mint_info = self.mint_info.get().expect("Mint info not found");

        // if public price > get account limit or create
        // if no public price > get whitelist limit or error
        let mint_state = match mint_info.public {
            0 => self.mint_state_list.get(&account_id).unwrap_or(MintState {
                limit: 0,
                listed: false,
            }),
            _ => self.mint_state_list.get(&account_id).unwrap_or(MintState {
                limit: mint_info.limit,
                listed: false,
            }),
        };

        //get the set of tokens for the passed in owner
        let tokens_for_owner_set = self.tokens_per_owner.get(&account_id);
        //if there is some token info return a state with some token
        if let Some(tokens) = tokens_for_owner_set {
            //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
            let start = u128::from(from_index.unwrap_or(0));
            //token state with info
            return JsonMintState {
                cost: match mint_state.listed {
                    true => mint_info.listed,
                    false => mint_info.public,
                },
                count: tokens.len(),
                limit: mint_state.limit,
                tokens: tokens
                    .iter()
                    //skip to the index we specified in the start variable
                    .skip(start as usize)
                    //take the first "limit" elements in the vector. If we didn't specify a limit, use 10
                    .take(limit.unwrap_or(10) as usize)
                    //we'll map the token IDs which are strings into Json Tokens
                    .map(|token_id| self.nft_token(token_id.clone()).unwrap())
                    //since we turned the keys into an iterator, we need to turn it back into a vector to return
                    .collect(),
            };
        }

        //if there is no set of tokens, we'll simply return an empty state.
        JsonMintState {
            cost: match mint_state.listed {
                true => mint_info.listed,
                false => mint_info.public,
            },
            count: 0,
            limit: mint_state.limit,
            tokens: Vec::new(),
        }
    }
}
