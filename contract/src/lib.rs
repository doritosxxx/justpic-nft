use near_sdk::collections::UnorderedSet;
use crate::metadata::NonFungibleTokenMetadata;
use crate::structs::NFTToken::{Token, TokenId};
use near_sdk::collections::LookupSet;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::AccountId;
use near_sdk::{env, near_bindgen, PanicOnDefault};

use crate::structs::NFTContractMetadata::NFTContractMetadata;

mod core;
mod enumeration;
mod metadata;
mod mint;
mod structs;
mod events;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
	owner_id: AccountId,
	metadata: LazyOption<NFTContractMetadata>,
	total_supply: u128,

	owner_by_id: LookupMap<TokenId, AccountId>,
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
			owner_by_id: LookupMap::new("byid".as_bytes()),
			owner_list: UnorderedSet::new("owners".as_bytes()),
			total_supply: 0,
		}
	}
}
