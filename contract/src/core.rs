use crate::AccountId;
use crate::Token;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::PromiseOrValue;

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
