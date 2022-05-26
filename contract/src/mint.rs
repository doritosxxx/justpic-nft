use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::AccountId;

pub trait NonFungibleTokenMint {
	fn nft_mint(&self, token_id: TokenId, receiver_id: AccountId);
}
