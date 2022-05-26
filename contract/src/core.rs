use crate::*;
use near_sdk::{near_bindgen, AccountId, PromiseOrValue};

pub trait NonFungibleTokenCore {
	fn nft_transfer(&self, receiver_id: AccountId, token_id: TokenId, memo: Option<String>);
	fn nft_transfer_call(
		&self,
		receiver_id: AccountId,
		token_id: TokenId,
		memo: Option<String>,
		msg: String,
	) -> PromiseOrValue<bool>;

	fn nft_token(&self, token_id: TokenId) -> Option<Token>;
}

#[near_bindgen]
impl NonFungibleTokenCore for Contract {
	fn nft_transfer(&self, receiver_id: AccountId, token_id: TokenId, memo: Option<String>) {
		todo!()
	}

	fn nft_transfer_call(
		&self,
		receiver_id: AccountId,
		token_id: TokenId,
		memo: Option<String>,
		msg: String,
	) -> PromiseOrValue<bool> {
		todo!()
	}

	fn nft_token(&self, token_id: TokenId) -> Option<Token> {
		self.tokens_by_id.get(&token_id)
	}
}
