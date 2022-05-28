use crate::structs::NFTToken::{Token, TokenId};
use crate::structs::TokenMetadata::TokenMetadata;
use crate::*;
use near_sdk::{near_bindgen, require, AccountId};

pub trait NonFungibleTokenMint {
	fn nft_mint(&mut self, token_id: TokenId, receiver_id: AccountId);
}

#[near_bindgen]
impl NonFungibleTokenMint for Contract {
	fn nft_mint(&mut self, token_id: TokenId, receiver_id: AccountId) {
		require!(
			!self.owner_by_id.contains_key(&token_id),
			"Token was already minted"
		);

		require!(
			!self.owner_list.contains(&receiver_id),
			"Can't have more than one token per account"
		);

		self.owner_by_id.insert(&token_id, &receiver_id);
		self.owner_list.insert(&receiver_id);
		self.total_supply += 1;
	}
}
