use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::Base64VecU8;
use near_sdk::serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTContractMetadata {
    // Required.
    pub spec: String,             // required, essentially a version like "nft-1.0.0"
    pub name: String,             // required, ex. "Mosaics"
    pub symbol: String,           // required, ex. "MOSIAC"
    pub base_uri: Option<String>, // Centralized gateway known to have reliable access to decentralized storage assets referenced by `reference` or `media` URLs
    // Optional.
    pub icon: Option<String>,                // Data URL
    pub reference: Option<String>,           // URL to a JSON file with more info
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}
