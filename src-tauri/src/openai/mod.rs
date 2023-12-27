use serde_json::{json, Value};

use self::openai_model::OpenAIModel;

pub mod chat_completion_model;
pub mod chat_completion_request;
pub mod chat_completion_response;
pub mod openai_model;

pub struct OpenAI {
    pub api_key: String,
}

impl OpenAI {
    pub fn new() -> Self {
        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

        Self { api_key }
    }

    pub fn chat_completion_request(
        &self,
        system_prompt: &str,
        user_prompt: &str,
        model: OpenAIModel,
    ) -> Result<String, reqwest::Error> {
        let client = reqwest::blocking::ClientBuilder::new()
            .timeout(None)
            .build()
            .expect("Failed to build reqwest client.");
        let body = json!({
            "model": model.to_string(),
            "messages": [
                {
                    "role": "system",
                    "content": system_prompt
                },
                {
                    "role": "user",
                    "content": user_prompt
                }
            ]
        });
        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .body(body.to_string())
            .send()?;

        if response.status().is_success() {
            let response_json: Value = response.json().expect("Failed to parse response.");
            let response_text = response_json["choices"][0]["message"]["content"]
                .as_str()
                .expect("Failed to parse response.");
            Ok(response_text.to_string())
        } else {
            panic!(
                "Failed to get a successful response: {:?}",
                response.status()
            );
        }
    }
}
