use serde_json::{json, Value};

use self::openai_model::OpenAIModel;
use self::chat_completion_request::ChatCompletionRequest;
use reqwest::{Client, ClientBuilder, header::HeaderMap};

pub mod chat_completion_model;
pub mod chat_completion_request;
pub mod chat_completion_response;
pub mod openai_client_error;

pub struct OpenAIClient {
    api_key: String,
    client: Client,
}

impl OpenAIClient {
    pub fn new() -> Self {
        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json");
        headers.insert("Authorization", format!("Bearer {}", api_key));

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .expect("Failed to initialize OpenAI Client.");

        Self { api_key, client }
    }
}

impl AIClient for OpenAIClient {
    pub async fn chat_completion_request(
        &self,
        request: ChatCompletionRequest
    ) -> Result<ChatCompletionResponse, OpenAIClientError> {
        let body = request.to_request_body();
        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .body(body)
            .send()
            .await.map_err(|e| OpenAIClientError::RequestFailed(format!("Error occurred making request to OpenAI:\n{}\n", e.to_string())))?;

        if response.status().is_success() {
            let response = response.json<ChatCompletionResponse>().map_err(|e| OpenAIClientError::InvalidResponse(format!("Failed to convert response into JSON:\n{}\n", e.to_string())))?;
            
            if !response.valid() {
                return Err(OpenAIClientError::InvalidResponse(format!("Received invalid finish reason '{}'.", response.choices[0].finish_reason)));
            }

            return Ok(response)
        } else {
            return Err(OpenAIClientError::BadStatus(format!("Client response status unsuccessful: {}",
            response.status())));
        }
    }
}
