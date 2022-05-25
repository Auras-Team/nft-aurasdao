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
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        receiver_id: AccountId,
        perpetual_royalties: Option<HashMap<AccountId, u32>>,
    ) {
        //storage so we need at least one yocto
        require_at_least_one_yocto();

        let sender_id = env::predecessor_account_id();

        //ensure that the predecessor has minting access
        let mint_count = self
            .allowed_list_mint
            .get(&sender_id)
            .expect("Account is not authorized to mint");
        require!(mint_count > 0, "Account has no mints remaining");

        // Verify pre-mint registration of token id
        require!(
            self.token_data_by_id.get(&token_id).is_some(),
            "Token id could not be found"
        );

        //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        //create a royalty map to store in the token
        let mut royalty = HashMap::new();

        //if perpetual royalties were passed into the function:
        if let Some(perpetual_royalties) = perpetual_royalties {
            //make sure that the length of the perpetual royalties is below 7 since we won't have enough GAS to pay out that many people
            require!(
                perpetual_royalties.len() < 7,
                "Cannot add more than 6 perpetual royalty amounts"
            );

            //used to verify that total is not larger then 10000
            let mut total_amount: u32 = 0;

            //iterate through the perpetual royalties and insert the account and amount in the royalty map
            for (account, amount) in perpetual_royalties {
                royalty.insert(account, amount);
                total_amount += amount;
            }

            //verify max base points to set 100.00 percentage values
            require!(
                total_amount <= 10000,
                "Total royalty shares can not be more then 10000 base points"
            );
        }

        //specify the token struct that contains the owner ID
        let token = Token {
            owner_id: receiver_id,
            approved_account_ids: Default::default(),
            next_approval_id: 0,
            royalty,
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
        self.allowed_list_mint.insert(&sender_id, &(mint_count - 1));

        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes);
    }
}
