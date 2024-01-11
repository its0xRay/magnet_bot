// src/openai/api.rs
use reqwest;
use serde_json::Value;
//use reqwest::header::InvalidHeaderValue;
//use std::error::Error;

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

        // Create the request headers with the API key
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", reqwest::header::HeaderValue::from_static("application/json"));

        let auth_header_value = reqwest::header::HeaderValue::from_str(&format!("Bearer {}", self.config.api_key))
         .map_err(|e| reqwest::Error::from(e).into())?;
        headers.insert("Authorization", auth_header_value);

        // Make the HTTP POST request to the OpenAI API
        let client = reqwest::blocking::Client::new();
        let response = client.post(api_url)
            .headers(headers)
            .body(reqwest::blocking::multipart::Form::new().text("prompt", prompt).text("max_tokens", max_tokens.to_string()))
            .send()?;

        // Check if the request was successful (status code 200)
        if response.status().is_success() {
            // Parse and return the response body
            let result: serde_json::Value = response.json()?;
            let choices = result.get("choices").unwrap();
            Ok(choices[0]["text"].as_str().unwrap_or_default().to_string())
        } else {
            // Return the error response
            Ok(response.json::<Value>()?.to_string())
        }
    }
}
