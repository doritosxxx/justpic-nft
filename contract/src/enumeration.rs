use crate::core::NonFungibleTokenCore;
use crate::*;
use near_sdk::{near_bindgen, AccountId};

use crate::structs::NFTToken::Token;

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

#[near_bindgen]
impl NonFungibleTokenEnumeration for Contract {
	fn nft_total_supply(&self) -> u128 {
		self.total_supply
	}

	fn nft_tokens(&self, from_index: Option<u128>, limit: Option<u128>) -> Vec<Token> {
		self.owner_list
			.iter()
			.map(|owner_id| Token::default(owner_id))
			.collect()
	}

	fn nft_supply_for_owner(&self, account_id: AccountId) -> u128 {
		if self.owner_list.contains(&account_id) {
			1
		} else {
			0
		}
	}

	fn nft_tokens_for_owner(
		&self,
		account_id: AccountId,
		from_index: Option<u128>,
		limit: Option<u128>,
	) -> Vec<Token> {
		if self.owner_list.contains(&account_id) {
			vec![Token::default(account_id)]
		} else {
			vec![]
		}
	}
}
