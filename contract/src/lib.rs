use crate::structs::NFTContractMetadata::NFTContractMetadata;
use crate::structs::NFTToken::Token;
use near_contract_standards::non_fungible_token::TokenId;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedSet};
use near_sdk::{near_bindgen, AccountId, PanicOnDefault};

mod core;
mod enumeration;
mod events;
mod metadata;
mod mint;
mod structs;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
	owner_id: AccountId,
	metadata: LazyOption<NFTContractMetadata>,
	total_supply: u128,

	owner_by_token_id: LookupMap<TokenId, AccountId>,
	owner_list: UnorderedSet<AccountId>,
}

#[near_bindgen]
impl Contract {
	#[init]
	pub fn new(owner_id: AccountId) -> Self {
		let metadata = NFTContractMetadata {
			spec: String::from("nft-1.0.0"), // required, essentially a version like "nft-1.0.0"
			name: String::from("nft-curse"), // required, ex. "Mosaics"
			symbol: String::from("CURSE"),   // required, ex. "MOSIAC"
			icon: None,                      // Data URL
			base_uri: None, // Centralized gateway known to have reliable access to decentralized storage assets referenced by `reference` or `media` URLs
			reference: None, // URL to a JSON file with more info
			reference_hash: None, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
		};

		let option: LazyOption<NFTContractMetadata> =
			LazyOption::new("meta".as_bytes(), Some(&metadata));
		Self {
			owner_id,
			metadata: option,
			owner_by_token_id: LookupMap::new("byid".as_bytes()),
			owner_list: UnorderedSet::new("owners".as_bytes()),
			total_supply: 0,
		}
	}
}
