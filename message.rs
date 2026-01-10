
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
  pub puzzle_number: u32,
  pub secret_key_number: u32,
}

impl Message {
  pub fn new(puzzle_number: u32, secret_key_number: u32) -> Self {
    Self { puzzle_number, secret_key_number }
  }
}
