use near_contract_standards::non_fungible_token::metadata::NFTContractMetadata;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::AccountId;
use near_sdk::{env, near_bindgen, PanicOnDefault};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]

pub struct Contract {
	owner_id: AccountId,
	// TODO: Convert to LazyOption<NFTContractMetadata>.
	metadata: NFTContractMetadata,
}

// near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);

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
		Self { metadata, owner_id }
	}

	pub fn clear_state() {
		env::storage_remove(b"STATE");
	}

	pub fn get_int() -> i32 {
		32
	}
}
