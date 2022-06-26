use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Counter { pub value:u64, }

#[near_bindgen]
impl Counter {
  pub fn get_num(&mut self) -> u64 { self.value }
  pub fn increment(&mut self) {
    self.value += 1;
    log!("Increased number to {}", self.value);
  }
  pub fn decrement(&mut self) {
    self.value -= 1;
    log!("Decreased number to {}", self.value);
  }
  pub fn reset(&mut self) {
    self.value = 0;
    log!("Reset number to {}", self.value);
  }
}

