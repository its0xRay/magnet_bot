// src/openai/config.rs
pub struct OpenAIConfig {
    pub api_key: String,
}

impl OpenAIConfig {
    pub fn new(api_key: String) -> Self {
        OpenAIConfig { api_key }
    }
}
