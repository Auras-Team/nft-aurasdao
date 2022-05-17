use crate::*;

pub trait NftRoyalties {
    //calculates the payout for a token given the passed in balance. This is a view method
    fn nft_payout(&self, token_id: TokenId, balance: U128, max_len_payout: u32) -> Payout;

    //transfers the token to the receiver ID and returns the payout object that should be payed given the passed in balance.
    fn nft_transfer_payout(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
        balance: U128,
        max_len_payout: u32,
    ) -> Payout;
}

fn _compute_payout(owner_id: AccountId, royalty: HashMap<AccountId, u32>, balance: u128) -> Payout {
    //keep track of the total perpetual royalties
    let mut total_perpetual = 0;
    //keep track of the payout object to send back
    let mut payout_object = Payout {
        payout: HashMap::new(),
    };

    //go through each key and value in the royalty object
    for (k, v) in royalty.iter() {
        //get the key
        let key = k.clone();
        //only insert into the payout if the key isn't the token owner (we add their payout at the end)
        if key != owner_id {
            //
            payout_object
                .payout
                .insert(key, royalty_to_payout(*v, balance));
            total_perpetual += *v;
        }
    }

    // payout to previous owner who gets 100% - total perpetual royalties
    payout_object.payout.insert(
        owner_id,
        royalty_to_payout(10000 - total_perpetual, balance),
    );

    //return the payout object
    payout_object
}

#[near_bindgen]
impl NftRoyalties for Contract {
    //calculates the payout for a token given the passed in balance. This is a view method
    fn nft_payout(&self, token_id: TokenId, balance: U128, max_len_payout: u32) -> Payout {
        //get the token object
        let token = self.tokens_by_id.get(&token_id).expect("Token not found");

        //make sure we're not paying out to too many people (GAS limits this)
        assert!(
            token.royalty.len() as u32 <= max_len_payout,
            "Market cannot payout to that many receivers"
        );

        // Compute and return the payout list
        _compute_payout(token.owner_id, token.royalty, u128::from(balance))
    }

    //transfers the token to the receiver ID and returns the payout object that should be payed given the passed in balance.
    #[payable]
    fn nft_transfer_payout(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
        balance: U128,
        max_len_payout: u32,
    ) -> Payout {
        //assert that the user attached 1 yocto NEAR for security reasons
        assert_one_yocto();
        //get the sender ID
        let sender_id = env::predecessor_account_id();
        //transfer the token to the passed in receiver and get the previous token object back
        let token = self.internal_transfer(&sender_id, &receiver_id, &token_id, approval_id, memo);

        //refund the previous token owner for the storage used up by the previous approved account IDs
        refund_approved_account_ids(token.owner_id.clone(), &token.approved_account_ids);

        //make sure we're not paying out to too many people (GAS limits this)
        assert!(
            token.royalty.len() as u32 <= max_len_payout,
            "Market cannot payout to that many receivers"
        );

        // Compute and return the payout list
        _compute_payout(token.owner_id, token.royalty, u128::from(balance))
    }
}
