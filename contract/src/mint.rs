use crate::events::log_mint;
use crate::structs::TokenId;
use crate::{Contract, ContractExt};
use near_sdk::{near_bindgen, require, AccountId};

pub trait NonFungibleTokenMint {
    fn nft_mint(&mut self, receiver_id: AccountId);
}

#[near_bindgen]
impl NonFungibleTokenMint for Contract {
    fn nft_mint(&mut self, receiver_id: AccountId) {
        require!(
            !self.owner_list.contains(&receiver_id),
            "Can't have more than one token per account"
        );

        let token_id: TokenId = self.next_mint_id.to_string();
        self.next_mint_id += 1;
        self.total_supply += 1;

        self.owner_by_token_id.insert(&token_id, &receiver_id);
        self.owner_list.insert(&receiver_id);

        log_mint(receiver_id, vec![token_id]);
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use crate::core::NonFungibleTokenCore;

    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, VMContext};

    fn get_context(signer: AccountId, is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id(signer)
            .is_view(is_view)
            .build()
    }

    #[test]
    fn mint_for_yourself() {
        let owner = accounts(0);
        let alice = accounts(1);
        testing_env!(get_context(alice.clone(), false));
        let mut contract = Contract::new(owner);

        contract.nft_mint(alice.clone());

        let minted_token = contract.nft_token("0".to_string());

        assert!(minted_token != None, "Token not minted");
        assert!(
            minted_token.unwrap().owner_id == alice,
            "Token receiver is not a token owner"
        )
    }

    #[test]
    fn mint_for_other() {
        let owner = accounts(0);
        let alice = accounts(1);
        let bob = accounts(2);
        testing_env!(get_context(alice.clone(), false));
        let mut contract = Contract::new(owner);

        contract.nft_mint(bob.clone());

        let minted_token = contract.nft_token("0".to_string());

        assert!(minted_token != None, "Token not minted");
        assert!(
            minted_token.unwrap().owner_id == bob,
            "Token receiver is not an owner"
        )
    }

    #[test]
    #[should_panic(expected = "Can't have more than one token per account")]
    fn mint_two_tokens_for_one_address() {
        let owner = accounts(0);
        let alice = accounts(1);
        testing_env!(get_context(alice.clone(), false));
        let mut contract = Contract::new(owner);

        contract.nft_mint(alice.clone());
        contract.nft_mint(alice.clone());
    }

    #[test]
    fn mint_tokens_for_different_addresses() {
        let owner = accounts(0);
        let alice = accounts(1);
        let bob = accounts(2);
        let charles = accounts(3);
        testing_env!(get_context(alice.clone(), false));
        let mut contract = Contract::new(owner);

        contract.nft_mint(alice.clone());
        contract.nft_mint(bob.clone());
        contract.nft_mint(charles.clone());

        let minted_token0 = contract.nft_token("0".to_string());
        let minted_token1 = contract.nft_token("1".to_string());
        let minted_token2 = contract.nft_token("2".to_string());

        assert!(minted_token0 != None, "Token 0 not minted");
        assert!(minted_token1 != None, "Token 1 not minted");
        assert!(minted_token2 != None, "Token 2 not minted");

        let token0 = minted_token0.unwrap();
        let token1 = minted_token1.unwrap();
        let token2 = minted_token2.unwrap();

        assert!(token0.token_id == "0", "Token 0 has id={}", token0.token_id);
        assert!(token1.token_id == "1", "Token 1 has id={}", token1.token_id);
        assert!(token2.token_id == "2", "Token 2 has id={}", token2.token_id);

        assert!(token0.owner_id == alice, "Token receiver is not an owner");
        assert!(token1.owner_id == bob, "Token receiver is not an owner");
        assert!(token2.owner_id == charles, "Token receiver is not an owner");
    }
}
