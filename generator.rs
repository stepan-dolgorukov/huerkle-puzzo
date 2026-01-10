use rand::rngs::OsRng;
use rand::{Rng, RngCore};
use sha2::Sha256;
use crate::message::{self};
use aes_gcm::{
  aead::{Aead, KeyInit},
  Aes256Gcm, Nonce,
};

use pbkdf2::pbkdf2_hmac;


fn generate_no() -> u32 {
  OsRng.r#gen()
}

fn generate_password(length: u8) -> String {
  let mut b = vec![0u8; length as usize];
  OsRng.fill_bytes(&mut b);
  hex::encode(b)
}

fn derive_key(password: &str) -> [u8; 32] {
  let mut key = [0u8; 32];
  pbkdf2_hmac::<Sha256>(password.as_bytes(), b"", 256, &mut key);
  key
}

fn generate_puzzle(password_length: u8) {
  let password = generate_password(password_length);
  let key = derive_key(&password);
  let puzzle_no = generate_no();
  println!("Puzzle no: {}", puzzle_no);
  let key_no = generate_no();
  println!("Key no: {}", key_no);
  let message = message::Message::new(puzzle_no, key_no);
  let nonce = Nonce::from_slice(&[0u8; 12]);
  let cipher = Aes256Gcm::new_from_slice(&key).unwrap();
  let serialized_message = bincode::serialize(&message).unwrap();
  let ciphertext = cipher
    .encrypt(nonce, serialized_message.as_ref())
    .unwrap();
  println!("Puzzle: {}", hex::encode(ciphertext));
  println!("Key: {}", hex::encode(key));
  println!("Password: {}", password);
  println!();
}

pub fn generate(puzzle_amount: u16, password_length: u8) {
  for _ in 0..(puzzle_amount - 1) {
    generate_puzzle(password_length);
  }
}
