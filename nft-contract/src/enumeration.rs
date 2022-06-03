use crate::*;

pub trait NftEnumeration {
    //get the total supply of NFTs on the contract
    fn nft_total_supply(&self) -> U128;

    //get nft tokens on the contract regardless of the owner using pagination
    fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonToken>;

    //get the total supply of NFTs for a given owner
    fn nft_supply_for_owner(&self, account_id: AccountId) -> U128;

    //get all the tokens for an owner using pagination
    fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<JsonToken>;
}

pub trait NftMintEnumeration {
    fn nft_registered_supply(&self) -> U128;

    fn nft_registered(
        &self,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> HashMap<String, TokenMetadata>;
}

#[near_bindgen]
impl NftEnumeration for Contract {
    //get the total supply of NFTs on the contract
    fn nft_total_supply(&self) -> U128 {
        //return the length of the token metadata by ID
        U128(self.tokens_by_id.len() as u128)
    }

    //get nft tokens on the contract regardless of the owner using pagination
    fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonToken> {
        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through each token using an iterator
        self.tokens_by_id
            .keys()
            //skip to the index we specified in the start variable
            .skip(start as usize)
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize)
            //we'll map the token IDs which are strings into Json Tokens
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }

    //get the total supply of NFTs for a given owner
    fn nft_supply_for_owner(&self, account_id: AccountId) -> U128 {
        //get the set of tokens for the passed in owner
        let tokens_for_owner_set = self.tokens_per_owner.get(&account_id);

        //if there is some set of tokens, we'll return the length as a U128
        if let Some(tokens_for_owner_set) = tokens_for_owner_set {
            return U128(tokens_for_owner_set.len() as u128);
        }
        //else if there isn't a set of tokens for the passed in account ID, we'll return 0
        U128(0)
    }

    //get all the tokens for an owner using pagination
    fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<JsonToken> {
        //get the set of tokens for the passed in owner
        let tokens_for_owner_set = self.tokens_per_owner.get(&account_id);
        //if there is some set of tokens, we'll set the tokens variable equal to that set
        let tokens = if let Some(tokens_for_owner_set) = tokens_for_owner_set {
            tokens_for_owner_set
        } else {
            //if there is no set of tokens, we'll simply return an empty vector.
            return vec![];
        };

        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through the keys vector
        tokens
            .iter()
            //skip to the index we specified in the start variable
            .skip(start as usize)
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize)
            //we'll map the token IDs which are strings into Json Tokens
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }
}

#[near_bindgen]
impl NftMintEnumeration for Contract {
    //get the registered supply of NFTs on the contract
    fn nft_registered_supply(&self) -> U128 {
        //return the length of the token metadata by ID
        U128(self.token_data_by_id.len() as u128)
    }

    //get the registered nft tokens for the contract
    fn nft_registered(
        &self,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> HashMap<String, TokenMetadata> {
        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //select the range of token keys using an iterator
        let list = self
            .token_data_by_id
            .keys()
            //skip to the index we specified in the start variable
            .skip(start as usize)
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize);

        let mut map = HashMap::new();

        for key in list {
            map.insert(
                key.clone(),
                self.token_data_by_id.get(&key.clone()).unwrap(),
            );
        }
        map
    }
}
