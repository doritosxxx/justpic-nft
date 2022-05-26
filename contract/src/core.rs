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
		let token = self.tokens_by_id.get(&token_id);
		require!(token != None, "Specified token_id does not exist");

		let mut token = token.unwrap();
		require!(
			env::predecessor_account_id() == token.owner_id,
			"Sender must be an owner of the token"
		);

		// Token exists and sender is owner.
		self.tokens_by_id.remove(&token_id);
		token.owner_id = receiver_id;
		self.tokens_by_id.insert(&token_id, &token);
	}

	fn nft_transfer_call(
		&mut self,
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
