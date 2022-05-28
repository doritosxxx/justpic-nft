use crate::events::*;
use crate::structs::NFTToken::Token;
use crate::structs::TokenMetadata::TokenMetadata;
use crate::{Contract, ContractExt};
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{env, near_bindgen, require, AccountId};

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

		log_mint(receiver_id, vec![token_id]);
	}
}

fn log_mint(owner_id: AccountId, token_ids: Vec<TokenId>) {
	let log = EventLog::new(LogOption::NftMint(vec![MintLog {
		owner_id,
		token_ids,
		memo: None,
	}]));

	env::log_str(&log.to_string());
}
