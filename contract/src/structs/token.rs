use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

use super::TokenMetadata;

pub type TokenId = String;

#[derive(Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize, PartialEq)]
#[cfg_attr(feature = "abi", derive(schemars::JsonSchema))]
#[serde(crate = "near_sdk::serde")]
pub struct Token {
    pub token_id: TokenId,
    pub owner_id: AccountId,
    pub metadata: TokenMetadata,
}

impl Token {
    pub fn default(owner_id: AccountId, token_id: TokenId) -> Self {
        Self {
            metadata: TokenMetadata::default(token_id.clone()),
            owner_id: owner_id,
            token_id,
        }
    }
}
