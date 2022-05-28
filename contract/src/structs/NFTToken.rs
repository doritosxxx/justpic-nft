use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

use crate::structs::TokenMetadata::TokenMetadata;

pub type TokenId = String;

#[derive(Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Token {
	pub token_id: TokenId,
	pub owner_id: AccountId,
	pub metadata: TokenMetadata,
}

impl Token {
	pub fn default(owner_id: AccountId) -> Self {
		Self {
			metadata: TokenMetadata::default(),
			owner_id: owner_id,
			token_id: "1".into(),
		}
	}
}
