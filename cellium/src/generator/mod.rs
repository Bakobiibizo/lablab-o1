use reqwest::Client;
use serde_json::Value;
use std::env;

pub struct Generator {
    client: Client,
    api_key: String,
    base_url: String,
    model: String,
}

impl Generator {
    pub fn new() -> Self {
        let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
        let base_url = env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
        let model = env::var("OPENAI_MODEL").unwrap_or_else(|_| "text-davinci-003".to_string());

        Self {
            client: Client::new(),
            api_key,
            base_url,
            model,
        }
    }

    pub async fn generate_text(&self, prompt: &str) -> Result<String, reqwest::Error> {
        // TODO: Implement OpenAI API interaction
        Ok(String::new())
    }
}