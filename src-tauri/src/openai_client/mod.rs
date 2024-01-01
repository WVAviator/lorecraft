use self::{
    chat_completion_request::ChatCompletionRequest,
    chat_completion_response::ChatCompletionResponse,
    image_generation::{
        image_generation_request::ImageGenerationRequest,
        image_generation_response::ImageGenerationResponse,
    },
    openai_client_error::OpenAIClientError,
};
use log::trace;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, ClientBuilder,
};

pub mod chat_completion_model;
pub mod chat_completion_request;
pub mod chat_completion_response;
pub mod image_generation;
pub mod openai_client_error;

pub struct OpenAIClient {
    client: Client,
}

impl OpenAIClient {
    pub fn new(api_key: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
        );

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .expect("Failed to initialize OpenAI Client.");

        Self { client }
    }
}

impl OpenAIClient {
    pub async fn verify_connection(&self) -> Result<(), OpenAIClientError> {
        let response = self
            .client
            .get("https://api.openai.com/v1/models")
            .send()
            .await
            .map_err(|e| {
                OpenAIClientError::RequestFailed(format!(
                    "Error occurred making request to OpenAI:\n{}\n",
                    e.to_string()
                ))
            })?;

        if response.status().is_client_error() {
            return Err(OpenAIClientError::NotAuthorized);
        }

        Ok(())
    }

    pub async fn chat_completion_request(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, OpenAIClientError> {
        let body = request.to_request_body();

        trace!(
            "Sending OpenAI chat completion request with body:\n\n{}\n\n",
            body
        );

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .body(body)
            .send()
            .await
            .map_err(|e| {
                OpenAIClientError::RequestFailed(format!(
                    "Error occurred making request to OpenAI:\n{}\n",
                    e.to_string()
                ))
            })?;

        trace!(
            "Received OpenAI chat completion response:\n\n{:?}\n\n",
            response
        );

        if response.status().is_success() {
            let response = response
                .json::<ChatCompletionResponse>()
                .await
                .map_err(|e| {
                    OpenAIClientError::InvalidResponse(format!(
                        "Failed to convert response into JSON:\n{}\n",
                        e.to_string()
                    ))
                })?;

            if !response.valid() {
                return Err(OpenAIClientError::InvalidResponse(format!(
                    "Received invalid finish reason '{}'.",
                    response.choices[0].finish_reason
                )));
            }

            return Ok(response);
        } else {
            return Err(OpenAIClientError::ResponseBadStatus(format!(
                "Client response status unsuccessful: {}",
                response.status()
            )));
        }
    }

    pub async fn image_generation_request(
        &self,
        request: ImageGenerationRequest,
    ) -> Result<ImageGenerationResponse, OpenAIClientError> {
        let body = request.to_request_body();

        trace!(
            "Sending OpenAI image generation request with body:\n\n{}\n\n",
            body
        );

        let response = self
            .client
            .post("https://api.openai.com/v1/images/generations")
            .body(body)
            .send()
            .await
            .map_err(|e| {
                OpenAIClientError::RequestFailed(format!(
                    "Error occurred making request to OpenAI:\n{}\n",
                    e.to_string()
                ))
            })?;

        trace!(
            "Received OpenAI image generation response:\n\n{:?}\n\n",
            response
        );

        if response.status().is_success() {
            let response = response
                .json::<ImageGenerationResponse>()
                .await
                .map_err(|e| {
                    OpenAIClientError::InvalidResponse(format!(
                        "Failed to convert response into JSON:\n{}\n",
                        e.to_string()
                    ))
                })?;

            return Ok(response);
        } else {
            return Err(OpenAIClientError::ResponseBadStatus(format!(
                "Client response status unsuccessful: {}",
                response.status()
            )));
        }
    }
}
