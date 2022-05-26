use crate::structs::NFTToken::{Token, TokenId};
use crate::*;
use near_sdk::{near_bindgen, AccountId};

pub trait NonFungibleTokenMint {
	fn nft_mint(&mut self, token_id: TokenId, receiver_id: AccountId);
}

#[near_bindgen]
impl NonFungibleTokenMint for Contract {
	fn nft_mint(&mut self, token_id: TokenId, receiver_id: AccountId) {
		let token = Token {
			owner_id: receiver_id,
			token_id: token_id.clone(),
		};
		self.tokens_by_id.insert(&token_id.into(), &token);
	}
}
