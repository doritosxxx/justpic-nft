use crate::events::log_transfer;
use crate::mint::NonFungibleTokenMint;
use crate::{structs::Token, Contract, ContractExt};
use near_sdk::{assert_one_yocto, env, near_bindgen, require, AccountId, PromiseOrValue};

use crate::structs::TokenId;

pub trait NonFungibleTokenCore {
    fn nft_transfer(&mut self, receiver_id: AccountId, token_id: TokenId, memo: Option<AccountId>);
    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool>;

    fn nft_token(&self, token_id: TokenId) -> Option<Token>;
}

#[near_bindgen]
impl NonFungibleTokenCore for Contract {
    /**
     * Use `memo` as `dupe_receiver_id` to specify second receiver.
     */
    #[payable]
    fn nft_transfer(&mut self, receiver_id: AccountId, token_id: TokenId, memo: Option<AccountId>) {
        assert_one_yocto();

        // Assert in runtime to provide meaningful error message.
        require!(
            memo != None,
            "memo should provide second address to transfer."
        );
        let dupe_receiver_id = memo.unwrap();
        let caller = env::predecessor_account_id();

        require!(
            Some(caller.clone()) == self.owner_by_token_id.get(&token_id),
            "Sender must be the token owner.",
        );

        require!(
            caller != receiver_id && caller != dupe_receiver_id,
            "Cant transfer to sender."
        );

        require!(receiver_id != dupe_receiver_id, "Receivers must differ.");

        require!(
            !self.owner_list.contains(&receiver_id) && !self.owner_list.contains(&dupe_receiver_id),
            "Receivers can't have more than one token per account"
        );

        // Trasnfer to `receiver_id`.
        self.owner_by_token_id.remove(&token_id);
        self.owner_by_token_id.insert(&token_id, &receiver_id);

        self.owner_list.remove(&caller);
        self.owner_list.insert(&receiver_id);
        log_transfer(caller, receiver_id, vec![token_id]);

        // Mint for `dupe_receiver_id`.
        self.nft_mint(dupe_receiver_id);
    }

    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool> {
        todo!("disabled")
    }

    fn nft_token(&self, token_id: TokenId) -> Option<Token> {
        let owner = self.owner_by_token_id.get(&token_id);

        if let None = owner {
            return None;
        }

        let owner_id = owner.unwrap();
        return Some(Token::default(owner_id, token_id));
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
            .attached_deposit(1)
            .build()
    }

    #[test]
    #[should_panic(expected = "memo should provide second address to transfer.")]

    fn transfer_with_one_receiver() {
        let owner = accounts(0);
        let alice = accounts(1);
        let bob = accounts(2);
        testing_env!(get_context(alice.clone(), false));
        let mut contract = Contract::new(owner);

        contract.nft_mint(alice.clone());

        let minted_token = contract.nft_token("0".to_string());

        if let Some(token) = minted_token {
            assert!(
                token.token_id == "0",
                "First minted toked_id is expected to be 0"
            );
        } else {
            panic!("Token was not minted");
        }

        contract.nft_transfer(bob, "0".into(), None);
    }
}
