// src/main.rs
mod openai;
mod chatbot;

use dotenv::dotenv;
use std::env;

fn main() {
    // Load environment variables from a .env file
    dotenv().ok();

    // Get the OpenAI API key from the environment variables
    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found in .env");

    // Create OpenAI configuration
    let openai_config = openai::config::OpenAIConfig::new(openai_api_key);

    // Create OpenAI API instance
    let openai_api = openai::api::OpenAIAPI::new(openai_config);

    println!("Hello! I'm your friend chatbot. Type 'exit' to end the conversation.");

    loop {
        // Get user input
        let user_input = chatbot::user_input::get_user_input();

        // Check for exit command
        if user_input.to_lowercase() == "exit" {
            println!("Goodbye! Have a great day!");
            break;
        }

        // Fetch response from OpenAI API
        let openai_response = openai_api.fetch_response(&user_input);

        // Generate and print chatbot response
        let chatbot_response = chatbot::responses::generate_response(&user_input);
        println!("Chatbot's response: {}", chatbot_response);
    }
}
