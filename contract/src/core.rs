use crate::events::{log_mint, log_transfer};
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

    // TODO:
    // fn nft_resolve_transfer
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
