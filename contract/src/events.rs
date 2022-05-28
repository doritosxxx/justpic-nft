use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json;
use std::fmt;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
struct EventLog {
	standard: String,
	version: String,
	#[serde(flatten)]
	data: LogOption,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
enum LogOption {
	NftMint(Vec<MintLog>),
	NftTransfer(Vec<TransferLog>),
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
struct MintLog {
	owner_id: String,
	token_ids: Vec<String>,
	memo: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
struct TransferLog {
	authorized_id: Option<String>,
	old_owner_id: String,
	new_owner_id: String,
	token_ids: Vec<String>,
	memo: Option<String>,
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
fn ser() {
	//let log = EventLog::new(data: LogOption)
	let log = EventLog::new(LogOption::NftMint(vec![MintLog {
		owner_id: String::from("alice.near"),
		token_ids: vec![String::from("aurora")],
		memo: None,
	}]));

	eprintln!("{}", log);

	let expected = r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"nft_mint","data":[{"owner_id":"alice.near","token_ids":["aurora"],"memo":null}]}"#;
	assert_eq!(expected, log.to_string());
}
