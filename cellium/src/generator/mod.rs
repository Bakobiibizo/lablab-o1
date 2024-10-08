use reqwest::Client;
use serde_json::Value;
use std::env;
use std::io::Error;
use std::io::ErrorKind;

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
        let request_body = serde_json::json!({
            "model": self.model,
            "prompt": prompt,
            "max_tokens": 150,
            "temperature": 0.7,
            "n": 1,
            "stop": null
        });

        let response = self
            .client
            .post(format!("{}/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body)
            .send()
            .await?;

        let response_json: Value = response.json().await?;

        // Extract the generated text from the response
        if let Some(text) = response_json["choices"][0]["text"].as_str() {
            Ok(text.trim().to_string())
        } else {
            Err(reqwest::Error::new(
                reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get generated text",
            ))
        }
    }
}