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
