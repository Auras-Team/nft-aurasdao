use near_sdk::require;

use crate::*;

/***************/
/* Pre Minting */
/***************/

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
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(&mut self, token_id: TokenId, receiver_id: AccountId) {
        //storage so we need at least one yocto
        require_at_least_one_yocto();

        let sender_id = env::predecessor_account_id();

        //ensure that the predecessor can mint tokens
        let mint_info = self.mint_state.get().expect("Minting is disabled");

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
                env::attached_deposit() >= ONE_NEAR * mint_info.listed,
                format!(
                    "Insufishend deposit, minting cost is {} near",
                    mint_info.listed
                )
            ),
            false => require!(
                env::attached_deposit() >= ONE_NEAR * mint_info.public,
                format!(
                    "Insufishend deposit, minting cost is {} near",
                    mint_info.public
                )
            ),
        }

        // Verify pre-mint registration of token id
        require!(
            self.token_data_by_id.get(&token_id).is_some(),
            "Token id could not be found"
        );

        //specify the token struct that contains the owner ID
        let token = Token {
            owner_id: receiver_id,
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
    }
}
