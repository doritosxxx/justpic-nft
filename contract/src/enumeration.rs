pub trait NonFungibleTokenEnumeration {
	fn nft_total_supply(&self) -> u128;
	fn nft_tokens(&self, from_index: Option<u128>, limit: Option<u128>) -> Vec<Token>;
	fn nft_supply_for_owner(&self, account_id: AccountId) -> u128;
	fn nft_tokens_for_owner(
		&self,
		account_id: AccountId,
		from_index: Option<u128>,
		limit: Option<u128>,
	) -> Vec<Token>;
}

