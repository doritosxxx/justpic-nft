use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json;
use std::fmt;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
struct EventLog {
	standard: String,
	version: String,
	event: String,
	//#[serde(flatten)]
	data: LogOption,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
//#[serde(tag = "event", content = "data")]
#[serde(untagged)]
enum LogOption {
	Mint(Vec<MintLog>),
	Transfer(Vec<TransferLog>),
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

fn get_event_string(data: &LogOption) -> String {
	let str = match data {
		LogOption::Mint(_) => "nft_mint",
		LogOption::Transfer(_) => "nft_transfer",
	};
	String::from(str)
}

impl EventLog {
	pub fn new(data: LogOption) -> Self {
		Self {
			standard: String::from("nep171"),
			version: String::from("1.0.0"),
			event: get_event_string(&data),
			data: data,
		}
	}
}

#[test]
fn ser() {
	//let log = EventLog::new(data: LogOption)
	let log = EventLog::new(LogOption::Mint(vec![MintLog {
		owner_id: String::from("alice.near"),
		token_ids: vec![String::from("aurora")],
		memo: None,
	}]));

	eprintln!("{}", log);

	let expected = r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"nft_mint","data":[{"owner_id":"alice.near","token_ids":["aurora"],"memo":null}]}"#;
	assert_eq!(expected, log.to_string());
}
