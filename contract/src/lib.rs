use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::Gas;
use near_sdk::{env, near_bindgen};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Contract {
	iterations: u128,
}

#[near_bindgen]
impl Contract {
	pub fn run(&mut self, reserved: u64) -> u128 {
		env::log(format!("Prepaid: {}", env::prepaid_gas()).as_bytes());

		let mut last: Gas = env::used_gas();
		self.iterations = 0;
		loop {
			if env::used_gas() + reserved * (env::used_gas() - last) > env::prepaid_gas() {
				env::log(format!("Iterations: {}", self.iterations).as_bytes());
				return self.iterations;
			}
			self.iterations += 1;
			last = env::used_gas();
		}
	}

	pub fn iterations(self) -> u128 {
		self.iterations
	}
}
