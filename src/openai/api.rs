// src/openai/api.rs
use reqwest;

pub struct OpenAIAPI {
    config: super::config::OpenAIConfig,
}

impl OpenAIAPI {
    pub fn new(config: super::config::OpenAIConfig) -> Self {
        OpenAIAPI { config }
    }

    pub fn fetch_response(&self, prompt: &str) -> Result<String, reqwest::Error> {
        // Set up the API endpoint URL
        let api_url = "https://api.openai.com/v1/engines/davinci-codex/completions";

        // Create the request payload
        let max_tokens = 50;
        let payload = json!({
            "prompt": prompt,
            "max_tokens": max_tokens,
        });

        // Create the request headers with the API key
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", reqwest::header::HeaderValue::from_static("application/json"));
        headers.insert("Authorization", reqwest::header::HeaderValue::from_str(&format!("Bearer {}", self.config.api_key))?);

        // Make the HTTP POST request to the OpenAI API
        let client = reqwest::blocking::Client::new();
        let response = client.post(api_url)
            .headers(headers)
            .json(&payload)
            .send()?;

        // Check if the request was successful (status code 200)
        if response.status().is_success() {
            // Parse and return the response body
            let result: serde_json::Value = response.json()?;
            let choices = result.get("choices").unwrap();
            Ok(choices[0]["text"].as_str().unwrap_or_default().to_string())
        } else {
            // Return the error response
            Err(reqwest::Error::from(response))
        }
    }
}
