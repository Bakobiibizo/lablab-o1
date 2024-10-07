use reqwest::Client;
use serde_json::Value;

pub struct Generator {
    client: Client,
    api_key: String,
}

impl Generator {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn generate_text(&self, prompt: &str) -> Result<String, reqwest::Error> {
        // TODO: Implement OpenAI API interaction
        Ok(String::new())
    }
}