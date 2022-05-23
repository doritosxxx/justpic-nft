use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Contract {
	iterations: u128,
}

#[near_bindgen]
impl Contract {
	pub fn run(&mut self) {
		env::log(format!("Deposit: {}", env::attached_deposit()).as_bytes());
		self.iterations = 0;
		loop {
			self.iterations += 1;
			if self.iterations == 1 {
				env::log("1".as_bytes());
			}
			if self.iterations == 10000 {
				env::log("10000".as_bytes());
			}
		}
	}

	pub fn iterations(self) -> u128 {
		self.iterations
	}
}
