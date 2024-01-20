use reqwest;
use serde::{Deserialize, Serialize};

pub struct OpenAI {
    api_key: String,
}

impl OpenAI {
    pub fn new(api_key: String) -> Self {
        OpenAI { api_key }
    }

    pub async fn get_chat_response(&self, user_input: &str) -> String {
        let prompt = format!("You: {}\nBot:", user_input);

        let response = reqwest::Client::new()
            .post("https://api.openai.com/v1/engines/davinci-codex/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&Request { prompt })
            .send()
            .await
            .expect("Failed to send request to OpenAI API")
            .json::<Response>()
            .await
            .expect("Failed to parse response from OpenAI API");

        response.choices[0].text.trim().to_string()
    }
}

#[derive(Debug, Serialize)]
struct Request {
    prompt: String,
}

#[derive(Debug, Deserialize)]
struct Response {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    text: String,
}
