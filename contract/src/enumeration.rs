use crate::structs::{Token, TokenId};
use crate::{Contract, ContractExt};
use near_sdk::{near_bindgen, AccountId};

pub trait NonFungibleTokenEnumeration {
    fn nft_total_supply(&self) -> u128;
    fn nft_tokens(&self, from_index: Option<TokenId>, limit: Option<u128>) -> Vec<Token>;
    fn nft_supply_for_owner(&self, account_id: AccountId) -> u128;
    fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<String>,
        limit: Option<u128>,
    ) -> Vec<Token>;
}

fn parse_from_limit(from: Option<TokenId>, limit: Option<u128>) -> (u128, u128) {
    let from: u128 = from.unwrap_or("0".into()).parse::<u128>().unwrap_or(0);
    let limit = limit.unwrap_or(u128::MAX);

    (from, limit)
}

#[near_bindgen]
impl NonFungibleTokenEnumeration for Contract {
    fn nft_total_supply(&self) -> u128 {
        self.total_supply
    }

    fn nft_tokens(&self, from_index: Option<TokenId>, limit: Option<u128>) -> Vec<Token> {
        let (from, limit) = parse_from_limit(from_index, limit);

        let mut tokens: Vec<Token> = Vec::new();

        for offset in 0..limit {
            let token_id = (from + offset).to_string();
            let owner_id = self.owner_by_token_id.get(&token_id);
            if let Some(owner_id) = owner_id {
                let token = Token::default(owner_id, token_id);
                tokens.push(token);
            } else {
                break;
            }
        }

        return tokens;
    }

    fn nft_supply_for_owner(&self, account_id: AccountId) -> u128 {
        if self.owner_list.contains(&account_id) {
            1
        } else {
            0
        }
    }

    fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<TokenId>,
        limit: Option<u128>,
    ) -> Vec<Token> {
        let (from, limit) = parse_from_limit(from_index, limit);

        if !self.owner_list.contains(&account_id) || from != 0 || limit == 0 {
            return vec![];
        } else {
            // Тут надо id поправить.
            return vec![Token::default(account_id, "0".into())];
        }
    }
}
