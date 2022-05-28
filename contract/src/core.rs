use crate::*;
use near_sdk::{env, near_bindgen, require, AccountId, PromiseOrValue};

pub trait NonFungibleTokenCore {
	fn nft_transfer(&mut self, receiver_id: AccountId, token_id: TokenId, memo: Option<String>);
	fn nft_transfer_call(
		&mut self,
		receiver_id: AccountId,
		token_id: TokenId,
		memo: Option<String>,
		msg: String,
	) -> PromiseOrValue<bool>;

	fn nft_token(&self, token_id: TokenId) -> Option<Token>;

	// TODO:
	// fn nft_resolve_transfer
}

#[near_bindgen]
impl NonFungibleTokenCore for Contract {
	fn nft_transfer(&mut self, receiver_id: AccountId, token_id: TokenId, memo: Option<String>) {
		
	}

	fn nft_transfer_call(
		&mut self,
		receiver_id: AccountId,
		token_id: TokenId,
		memo: Option<String>,
		msg: String,
	) -> PromiseOrValue<bool> {
		PromiseOrValue::Value(true)
		//todo!("disabled")
	}

	fn nft_token(&self, token_id: TokenId) -> Option<Token> {
		let owner = self.owner_by_id.get(&token_id);

		if let None = owner {
			return None;
		}

		let owner_id = owner.unwrap();
		return Some(Token::default(owner_id));
	}
}
