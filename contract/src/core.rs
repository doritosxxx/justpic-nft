use crate::{structs::Token, Contract, ContractExt};
use near_sdk::{assert_one_yocto, env, near_bindgen, require, AccountId, PromiseOrValue};

use crate::structs::TokenId;

pub trait NonFungibleTokenCore {
    fn nft_transfer(&mut self, receiver_id: AccountId, token_id: TokenId, memo: Option<String>);
    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool>;

    fn nft_token(&self, token_id: TokenId) -> Option<Token>;

    // TODO:
    // fn nft_resolve_transfer
}

#[near_bindgen]
impl NonFungibleTokenCore for Contract {
    #[payable]
    fn nft_transfer(&mut self, receiver_id: AccountId, token_id: TokenId, memo: Option<String>) {
        assert_one_yocto();
        require!(
            self.owner_by_token_id.contains_key(&token_id),
            "Sender must be the token owner.",
        );

        self.owner_by_token_id.remove(&token_id);
        self.owner_by_token_id.insert(&token_id, &receiver_id);

        self.owner_list.remove(&env::predecessor_account_id());
        self.owner_list.insert(&receiver_id);
    }

    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool> {
        PromiseOrValue::Value(true)
        //todo!("disabled")
    }

    fn nft_token(&self, token_id: TokenId) -> Option<Token> {
        let owner = self.owner_by_token_id.get(&token_id);

        if let None = owner {
            return None;
        }

        let owner_id = owner.unwrap();
        return Some(Token::default(owner_id));
    }
}
