// src/chatbot/user_input.rs
use std::io;

pub fn get_user_input() -> String {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    user_input.trim().to_string()
}
