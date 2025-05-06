pub mod game_engine;
pub mod world;
pub mod characters;
pub mod items;
pub mod events;
pub mod progression;
pub mod dialogue;
pub mod utils;
pub mod scenario;

use std::io::{self, Write};

pub fn prompt<T: std::str::FromStr>(msg: &str, default: T) -> T {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        if let Ok(val) = input.trim().parse::<T>() {
            return val;
        }
    }
    default
}

pub fn prompt_string(msg: &str, default: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let s = input.trim();
        if !s.is_empty() {
            return s.to_string();
        }
    }
    default.to_string()
}