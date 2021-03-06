use crate::metadata::NonFungibleTokenMetadata;
use crate::structs::NFTToken::{Token, TokenId};

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::AccountId;
use near_sdk::{env, near_bindgen, PanicOnDefault};

use crate::structs::NFTContractMetadata::NFTContractMetadata;

mod core;
mod metadata;
mod mint;
mod structs;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
	owner_id: AccountId,
	metadata: LazyOption<NFTContractMetadata>,
	tokens_by_id: LookupMap<TokenId, Token>,
}

#[near_bindgen]
impl Contract {
	#[init]
	pub fn new(owner_id: AccountId) -> Self {
		let metadata = NFTContractMetadata {
			spec: String::from("nft-1.0.0"), // required, essentially a version like "nft-1.0.0"
			name: String::from("justpic"),   // required, ex. "Mosaics"
			symbol: String::from("JPIC"),    // required, ex. "MOSIAC"
			icon: None,                      // Data URL
			base_uri: None, // Centralized gateway known to have reliable access to decentralized storage assets referenced by `reference` or `media` URLs
			reference: None, // URL to a JSON file with more info
			reference_hash: None, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
		};

		let option: LazyOption<NFTContractMetadata> =
			LazyOption::new(b"meta".try_to_vec().unwrap(), Some(&metadata));
		Self {
			metadata: option,
			owner_id,
			tokens_by_id: LookupMap::new("byid".as_bytes()),
		}
	}

	pub fn clear_state(&self) {
		env::storage_remove(b"STATE");
	}

	pub fn get_int(&self) -> i32 {
		32
	}
}

#[test]
fn test_get_metadata() {
	let contract = Contract::new("doritosxxx3.testnet".parse().unwrap());

	let metadata = contract.nft_metadata();

	assert_eq!(String::from("justpic"), metadata.name);
}
