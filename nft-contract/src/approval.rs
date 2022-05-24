use crate::*;
use near_sdk::{ext_contract, require, Gas};

const GAS_FOR_NFT_APPROVE: Gas = Gas(10_000_000_000_000);
const NO_DEPOSIT: Balance = 0;

pub trait NftApproval {
    //approve an account ID to transfer a token on your behalf
    fn nft_approve(&mut self, token_id: TokenId, account_id: AccountId, msg: Option<String>);

    //check if the passed in account has access to approve the token ID
    fn nft_is_approved(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) -> bool;

    //revoke a specific account from transferring the token on your behalf
    fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId);

    //revoke all accounts from transferring the token on your behalf
    fn nft_revoke_all(&mut self, token_id: TokenId);
}

#[ext_contract(ext_non_fungible_approval_receiver)]
trait NonFungibleTokenApprovalsReceiver {
    //cross contract call to an external contract that is initiated during nft_approve
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    );
}

#[near_bindgen]
impl NftApproval for Contract {
    //allow a specific account ID to approve a token on your behalf
    #[payable]
    fn nft_approve(&mut self, token_id: TokenId, account_id: AccountId, msg: Option<String>) {
        //require at least one yocto for security reasons and to  pay for storage on the contract
        require_at_least_one_yocto();

        //get the token object from the token ID
        let mut token = self.tokens_by_id.get(&token_id).expect("Token not found");

        //make sure that the person calling the function is the owner of the token
        require!(
            &env::predecessor_account_id() == &token.owner_id,
            "Only owner can approve transfer access",
        );

        //get the next approval ID if we need a new approval
        let approval_id: u64 = token.next_approval_id;

        //check if the account has been approved already for this token
        let is_new_approval = token
            .approved_account_ids
            //insert returns none if the key was not present.
            .insert(account_id.clone(), approval_id)
            //if the key was not present, .is_none() will return true so it is a new approval.
            .is_none();

        //if it was a new approval, we need to calculate how much storage is being used to add the account.
        let storage_used = if is_new_approval {
            bytes_for_approved_account_id(&account_id)
        //if it was not a new approval, we used no storage.
        } else {
            0
        };

        //increment the token's next approval ID by 1
        token.next_approval_id += 1;
        //insert the token back into the tokens_by_id collection
        self.tokens_by_id.insert(&token_id, &token);

        //refund any excess storage attached by the user. If the user didn't attach enough, panic.
        refund_deposit(storage_used);

        //if some message was passed into the function, we initiate a cross contract call on the
        //account we're giving access to.
        if let Some(msg) = msg {
            ext_non_fungible_approval_receiver::nft_on_approve(
                token_id,
                token.owner_id,
                approval_id,
                msg,
                account_id,
                NO_DEPOSIT,
                env::prepaid_gas() - GAS_FOR_NFT_APPROVE,
            )
            .as_return();
        }
    }

    //check if the passed in account has access to approve the token ID
    fn nft_is_approved(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) -> bool {
        //get and check the token object from the token_id
        if let Some(token) = self.tokens_by_id.get(&token_id) {
            //if there was some approval ID found for the account ID
            if let Some(approval) = token.approved_account_ids.get(&approved_account_id) {
                //if a specific approval_id was passed into the function
                if let Some(approval_id) = approval_id {
                    //return if the approval ID passed in matches the actual approval ID for the account
                    return approval_id == *approval;
                } else {
                    //if there was no approval_id passed into the function, we simply return true
                    return true;
                }
            }
        }
        //if there was no token or approval ID found
        //for the account ID, we simply return false
        false
    }

    //revoke a specific account from transferring the token on your behalf
    #[payable]
    fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId) {
        //require that the user attached exactly 1 yoctoNEAR for security reasons
        require_one_yocto();

        //get the token object using the passed in token_id
        let mut token = self.tokens_by_id.get(&token_id).expect("Token not found");

        //get the caller of the function and require that they are the owner of the token
        let predecessor_account_id = env::predecessor_account_id();
        require!(
            &env::predecessor_account_id() == &token.owner_id,
            "Only owner can revoke transfer access",
        );

        //if the account ID was in the token's approval, we remove it and the if statement logic executes
        if token.approved_account_ids.remove(&account_id).is_some() {
            //refund the funds released by removing the approved_account_id to the caller of the function
            refund_approved_account_ids_iter(predecessor_account_id, [account_id].iter());

            //insert the token back into the tokens_by_id collection with the account_id removed from the approval list
            self.tokens_by_id.insert(&token_id, &token);
        }
    }

    //revoke all accounts from transferring the token on your behalf
    #[payable]
    fn nft_revoke_all(&mut self, token_id: TokenId) {
        //require that the caller attached exactly 1 yoctoNEAR for security
        require_one_yocto();

        //get the token object from the passed in token ID
        let mut token = self.tokens_by_id.get(&token_id).expect("Token not found");

        //get the caller and make sure they are the owner of the tokens
        let predecessor_account_id = env::predecessor_account_id();
        require!(
            &predecessor_account_id == &token.owner_id,
            "Only owner can revoke all transfer access",
        );

        //only revoke if the approved account IDs for the token is not empty
        if !token.approved_account_ids.is_empty() {
            //refund the approved account IDs to the caller of the function
            refund_approved_account_ids(predecessor_account_id, &token.approved_account_ids);
            //clear the approved account IDs
            token.approved_account_ids.clear();
            //insert the token back into the tokens_by_id collection with the approved account IDs cleared
            self.tokens_by_id.insert(&token_id, &token);
        }
    }
}
