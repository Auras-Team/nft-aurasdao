use near_sdk::require;

use crate::*;

/// Royalty amount in basepoints for the contract owner
pub const MANAGER_ROYALTY: u32 = 600;

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

fn _compute_payout(owner_id: AccountId, manager_id: AccountId, balance: u128) -> Payout {
    //keep track of the payout object to send back
    let mut payout_object = Payout {
        payout: HashMap::new(),
    };

    // payout to previous owner who gets 100% - 6% managment fees
    payout_object
        .payout
        .insert(manager_id, royalty_to_payout(MANAGER_ROYALTY, balance));
    payout_object.payout.insert(
        owner_id,
        royalty_to_payout(10000 - MANAGER_ROYALTY, balance),
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

        //make sure we're not paying out to too many people (GAS limits)
        require!(max_len_payout > 0, "Market cannot payout royalties");

        // Compute and return the payout list
        _compute_payout(token.owner_id, self.owner_id.clone(), u128::from(balance))
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
        //require that the user attached 1 yocto NEAR for security reasons
        require_one_yocto();

        //make sure we're not paying out to too many people (GAS limits)
        require!(max_len_payout > 0, "Market cannot payout royalties");

        //get the sender ID
        let sender_id = env::predecessor_account_id();
        //transfer the token to the passed in receiver and get the previous token object back
        let token = self.internal_transfer(&sender_id, &receiver_id, &token_id, approval_id, memo);

        //refund the previous token owner for the storage used up by the previous approved account IDs
        refund_approved_account_ids(token.owner_id.clone(), &token.approved_account_ids);

        // Compute and return the payout list
        _compute_payout(token.owner_id, self.owner_id.clone(), u128::from(balance))
    }
}
