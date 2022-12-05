use crate::events::log_mint;
use crate::structs::TokenId;
use crate::{Contract, ContractExt};
use near_sdk::{near_bindgen, require, AccountId};

pub trait NonFungibleTokenMint {
    fn nft_mint(&mut self, receiver_id: AccountId);
}

#[near_bindgen]
impl NonFungibleTokenMint for Contract {
    fn nft_mint(&mut self, receiver_id: AccountId) {
        require!(
            !self.owner_list.contains(&receiver_id),
            "Can't have more than one token per account"
        );

        let token_id: TokenId = self.next_mint_id.to_string();
        self.next_mint_id += 1;
        self.total_supply += 1;

        self.owner_by_token_id.insert(&token_id, &receiver_id);
        self.owner_list.insert(&receiver_id);

        log_mint(receiver_id, vec![token_id]);
    }
}
