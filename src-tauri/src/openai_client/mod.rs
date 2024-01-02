use self::{
    assisstant_api::{
        assisstant_create_request::AssisstantCreateRequest,
        assisstant_create_response::AssisstantCreateResponse,
    },
    chat_completion::chat_completion_request::ChatCompletionRequest,
    chat_completion::chat_completion_response::ChatCompletionResponse,
    image_generation::{
        image_generation_request::ImageGenerationRequest,
        image_generation_response::ImageGenerationResponse,
    },
    openai_client_error::OpenAIClientError,
    thread::thread_create_response::ThreadCreateResponse,
};
use log::{error, trace};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, ClientBuilder,
};

pub mod assisstant_api;
pub mod chat_completion;
pub mod image_generation;
pub mod openai_client_error;
pub mod openai_tool;
pub mod thread;

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

    pub async fn create_assisstant(
        &self,
        request: AssisstantCreateRequest,
    ) -> Result<AssisstantCreateResponse, OpenAIClientError> {
        let body = request.to_request_body();

        trace!(
            "Sending OpenAI assisstant create request with body:\n\n{}\n\n",
            body
        );

        let response = self
            .client
            .post("https://api.openai.com/v1/assistants")
            .header("OpenAI-Beta", "assisstants=v1")
            .body(body)
            .send()
            .await
            .map_err(|e| {
                OpenAIClientError::RequestFailed(format!(
                    "Error occurred making request to OpenAI:\n{}\n",
                    e.to_string()
                ))
            })?;

        if response.status().is_success() {
            let response = response
                .json::<AssisstantCreateResponse>()
                .await
                .map_err(|e| {
                    OpenAIClientError::InvalidResponse(format!(
                        "Failed to convert response into JSON:\n{}\n",
                        e.to_string()
                    ))
                })?;

            return Ok(response);
        } else {
            error!("Received bad status from OpenAI: {}", response.status());
            return Err(OpenAIClientError::ResponseBadStatus(format!(
                "Client response status unsuccessful: {}",
                response.status()
            )));
        }
    }

    pub async fn create_thread(&self) -> Result<ThreadCreateResponse, OpenAIClientError> {
        let response = self
            .client
            .post("https://api.openai.com/v1/threads")
            .header("OpenAI-Beta", "assistants=v1")
            .send()
            .await
            .map_err(|e| {
                OpenAIClientError::RequestFailed(format!(
                    "Error occurred making request to OpenAI:\n{}\n",
                    e.to_string()
                ))
            })?;

        if response.status().is_success() {
            let response = response.json::<ThreadCreateResponse>().await.map_err(|e| {
                OpenAIClientError::InvalidResponse(format!(
                    "Failed to convert response into JSON:\n{}\n",
                    e.to_string()
                ))
            })?;

            return Ok(response);
        } else {
            error!("Received bad status from OpenAI: {}", response.status());
            return Err(OpenAIClientError::ResponseBadStatus(format!(
                "Client response status unsuccessful: {}",
                response.status()
            )));
        }
    }
}

// curl https://api.openai.com/v1/threads \
//   -H "Content-Type: application/json" \
//   -H "Authorization: Bearer $OPENAI_API_KEY" \
//   -H "OpenAI-Beta: assistants=v1" \
//   -d ''
