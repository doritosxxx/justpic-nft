use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json;
use near_sdk::AccountId;
use std::fmt;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct EventLog {
	standard: String,
	version: String,
	#[serde(flatten)]
	data: LogOption,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
pub enum LogOption {
	NftMint(Vec<MintLog>),
	NftTransfer(Vec<TransferLog>),
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MintLog {
	pub owner_id: AccountId,
	pub token_ids: Vec<TokenId>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub memo: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TransferLog {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub authorized_id: Option<AccountId>,
	pub old_owner_id: AccountId,
	pub new_owner_id: AccountId,
	pub token_ids: Vec<TokenId>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub memo: Option<String>,
}

impl fmt::Display for EventLog {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_fmt(format_args!(
			"EVENT_JSON:{}",
			&serde_json::to_string(self).map_err(|_| fmt::Error)?
		))
	}
}

impl EventLog {
	pub fn new(data: LogOption) -> Self {
		Self {
			standard: String::from("nep171"),
			version: String::from("1.0.0"),
			data: data,
		}
	}
}

#[test]
fn mint_log() {
	let log = EventLog::new(LogOption::NftMint(vec![MintLog {
		owner_id: "alice.near".parse().unwrap(),
		token_ids: vec!["aurora".parse().unwrap()],
		memo: None,
	}]));

	let expected = r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"nft_mint","data":[{"owner_id":"alice.near","token_ids":["aurora"]}]}"#;
	assert_eq!(expected, log.to_string());
}

#[test]
fn mint_log_empty_tokens() {
	let log = EventLog::new(LogOption::NftMint(vec![MintLog {
		owner_id: "bob.near".parse().unwrap(),
		token_ids: vec![],
		memo: None,
	}]));

	let expected = r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"nft_mint","data":[{"owner_id":"bob.near","token_ids":[]}]}"#;
	assert_eq!(expected, log.to_string());
}

#[test]
fn mint_log_multiple_logs() {
	let log = EventLog::new(LogOption::NftMint(vec![
		MintLog {
			owner_id: "alice.near".parse().unwrap(),
			token_ids: vec!["to_alice".to_string()],
			memo: None,
		},
		MintLog {
			owner_id: "bob.near".parse().unwrap(),
			token_ids: vec!["to_bob".parse().unwrap()],
			memo: None,
		},
	]));

	let expected = r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"nft_mint","data":[{"owner_id":"alice.near","token_ids":["to_alice"]},{"owner_id":"bob.near","token_ids":["to_bob"]}]}"#;
	assert_eq!(expected, log.to_string());
}

#[test]
fn mint_log_multiple_tokens() {
	let log = EventLog::new(LogOption::NftMint(vec![MintLog {
		owner_id: "alice.near".parse().unwrap(),
		token_ids: vec![
			"first".parse().unwrap(),
			"second".parse().unwrap(),
			"3".parse().unwrap(),
		],
		memo: None,
	}]));

	let expected = r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"nft_mint","data":[{"owner_id":"alice.near","token_ids":["first","second","3"]}]}"#;
	assert_eq!(expected, log.to_string());
}
