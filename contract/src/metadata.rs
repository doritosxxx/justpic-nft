use crate::*;

pub trait NonFungibleTokenMetadata {
	fn nft_metadata(&self) -> NFTContractMetadata;
}

#[near_bindgen]
impl NonFungibleTokenMetadata for Contract {
	fn nft_metadata(&self) -> NFTContractMetadata {
		self.metadata.get().unwrap()
	}
}
