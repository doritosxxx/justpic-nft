:: near call dev-1653592161668-77636541836858 new "{\"owner_id\":\"hui3.testnet\"}" --accountId hui3.testnet
:: near view dev-1653592161668-77636541836858 nft_metadata
:: near call dev-1653592161668-77636541836858 clear_state --accountIdhui3.testnet
:: near call dev-1653592161668-77636541836858 nft_mint "{\"token_id\":\"2\",\"receiver_id\":\"hui3.testnet\"}" --accountId hui3.testnet
:: near call dev-1653592161668-77636541836858 nft_token "{\"token_id\":\"1\"}" --accountId hui3.testnet
near call dev-1653592161668-77636541836858 nft_tokens_for_owner "{\"account_id\":\"hui3.testnet\"}" --accountId hui3.testnet