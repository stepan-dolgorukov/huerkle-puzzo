use rand::rngs::OsRng;
use rand::{Rng, RngCore};
use sha2::Sha256;
use crate::message::{self, Message};
use aes_gcm::{
  aead::{Aead, KeyInit},
  Aes256Gcm, Nonce,
};
use pbkdf2::pbkdf2_hmac;

fn derive_key(password: &str) -> [u8; 32] {
  let mut key = [0u8; 32];
  pbkdf2_hmac::<Sha256>(password.as_bytes(), b"", 256, &mut key);
  key
}

pub fn solve(puzzle: [u8; 24], password_length: u8) {
  for password_as_number in 0..(1u32 << (password_length * 8)) {
    // println!("{}", password_as_number);
    let password = hex::encode(password_as_number.to_be_bytes());
    // println!("{}", password);
    let key = derive_key(&password);
    let cipher = Aes256Gcm::new_from_slice(&key).unwrap();
    let nonce= Nonce::from_slice(&[0u8; 12]);
    match cipher.decrypt(nonce, puzzle.as_ref()) {
      Ok(v) => {
        println!("Solved");
        let deserialized_message = bincode::deserialize::<Message>(&v).unwrap();
        println!("Puzzle number: {}", deserialized_message.puzzle_number);
        println!("Secret key number: {}", deserialized_message.secret_key_number);
        println!("Key: {}", hex::encode(key));
        return;
      }
      Err(_) => {
      }
    }
  }
}
