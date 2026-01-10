use std::io::{self, Read, Write};

use clap::{Parser, ValueEnum};

use crate::solver::solve;
mod generator;
mod message;
mod solver;

#[derive(Copy, Clone, ValueEnum, Debug)]
enum Mode {
  Generator,
  Solver,
}

#[derive(Parser)]
#[command(name = "hupu", version = "0.1.0")]
struct Arguments {
  #[arg(long, value_enum)]
  mode: Mode,
  #[arg(long, required_if_eq("mode", "generator"))]
  puzzle_amount: Option<u16>,
  #[arg(long)]
  password_length: u8,
}

fn main() {
  let arguments = Arguments::parse();
  match arguments.mode {
    Mode::Generator => { generator::generate(arguments.puzzle_amount.unwrap(), arguments.password_length); }
    Mode::Solver => {
      print!("Puzzle: ");
      let _ = io::stdout().flush();
      let mut input = String::new();
      io::stdin().read_line(&mut input).unwrap();
      let hex_str: String = input.split_whitespace().collect();
      let bytes = hex::decode(hex_str).unwrap();
      solve(bytes.try_into().unwrap(), arguments.password_length);
    }
  }
}
