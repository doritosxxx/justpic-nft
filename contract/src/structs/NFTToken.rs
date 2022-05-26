use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

pub type TokenId = String;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Token {
	pub token_id: TokenId,
	pub owner_id: AccountId,
}
