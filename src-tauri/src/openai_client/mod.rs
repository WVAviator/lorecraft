use std::fs;

use self::{
    assistant_create::{
        assistant_create_request::AssistantCreateRequest,
        assistant_create_response::AssistantCreateResponse,
    },
    chat_completion::chat_completion_request::ChatCompletionRequest,
    chat_completion::chat_completion_response::ChatCompletionResponse,
    create_message::{
        create_message_request::CreateMessageRequest,
        create_message_response::CreateMessageResponse,
    },
    create_run::{create_run_request::CreateRunRequest, create_run_response::CreateRunResponse},
    image_generation::{
        image_generation_request::ImageGenerationRequest,
        image_generation_response::ImageGenerationResponse,
    },
    list_messages::{
        list_messages_query::ListMessagesQuery, list_messages_response::ListMessagesResponse,
    },
    openai_client_error::OpenAIClientError,
    retrieve_run::retrieve_run_response::RetrieveRunResponse,
    submit_tool_outputs::submit_tool_outputs_request::SubmitToolOutputsRequest,
    thread_create::thread_create_response::ThreadCreateResponse,
};
use anyhow::Context;
use log::{error, trace};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, ClientBuilder,
};

pub mod assistant_create;
pub mod assistant_tool;
pub mod chat_completion;
pub mod create_message;
pub mod create_run;
pub mod image_generation;
pub mod list_messages;
pub mod openai_client_error;
pub mod retrieve_run;
pub mod submit_tool_outputs;
pub mod thread_create;

#[derive(Debug, Clone)]
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

    pub async fn create_assistant(
        &self,
        request: AssistantCreateRequest,
    ) -> Result<AssistantCreateResponse, OpenAIClientError> {
        let body = request.to_request_body();

        trace!(
            "Sending OpenAI assistant create request with body:\n\n{:#?}\n\n",
            body
        );

        let response = self
            .client
            .post("https://api.openai.com/v1/assistants")
            .header("OpenAI-Beta", "assistants=v1")
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
                .json::<AssistantCreateResponse>()
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

    pub async fn delete_assistant(&self, assistant_id: &str) -> Result<(), OpenAIClientError> {
        let url = format!("https://api.openai.com/v1/assistants/{}", assistant_id);

        let response = self
            .client
            .delete(&url)
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
            return Ok(());
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

    pub async fn delete_thread(&self, thread_id: &str) -> Result<(), OpenAIClientError> {
        let url = format!("https://api.openai.com/v1/threads/{}", thread_id);

        let response = self
            .client
            .delete(&url)
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
            return Ok(());
        } else {
            error!("Received bad status from OpenAI: {}", response.status());
            return Err(OpenAIClientError::ResponseBadStatus(format!(
                "Client response status unsuccessful: {}",
                response.status()
            )));
        }
    }

    pub async fn create_message(
        &self,
        message_request: CreateMessageRequest,
        thread_id: &str,
    ) -> Result<CreateMessageResponse, OpenAIClientError> {
        let body = message_request.to_request_body().map_err(|e| {
            error!("Invalid request body:\n{:?}", e);
            OpenAIClientError::RequestFailed(format!("Invalid request body."))
        })?;
        let url = format!("https://api.openai.com/v1/threads/{}/messages", thread_id);
        let response = self
            .client
            .post(&url)
            .header("OpenAI-Beta", "assistants=v1")
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
                .json::<CreateMessageResponse>()
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

    pub async fn list_messages(
        &self,
        query: ListMessagesQuery,
    ) -> Result<ListMessagesResponse, OpenAIClientError> {
        let response = self
            .client
            .get(&query.url)
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
            let response = response.json::<ListMessagesResponse>().await.map_err(|e| {
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

    pub async fn create_run(
        &self,
        request: CreateRunRequest,
        thread_id: &str,
    ) -> Result<CreateRunResponse, OpenAIClientError> {
        let body = request.to_request_body().map_err(|e| {
            error!("Invalid request body:\n{:?}", e);
            OpenAIClientError::RequestFailed(format!("Invalid request body."))
        })?;

        let url = format!("https://api.openai.com/v1/threads/{}/runs", thread_id);

        let response = self
            .client
            .post(&url)
            .header("OpenAI-Beta", "assistants=v1")
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
            let response = response.json::<CreateRunResponse>().await.map_err(|e| {
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

    pub async fn retrieve_run(
        &self,
        thread_id: &str,
        run_id: &str,
    ) -> Result<RetrieveRunResponse, OpenAIClientError> {
        let url = format!(
            "https://api.openai.com/v1/threads/{}/runs/{}",
            thread_id, run_id
        );
        let response = self
            .client
            .get(&url)
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
            let response = response.json::<RetrieveRunResponse>().await.map_err(|e| {
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

    pub async fn submit_tool_outputs(
        &self,
        request: SubmitToolOutputsRequest,
        thread_id: &str,
        run_id: &str,
    ) -> Result<RetrieveRunResponse, OpenAIClientError> {
        let body = request.to_request_body().map_err(|e| {
            error!("Invalid request body:\n{:?}", e);
            OpenAIClientError::RequestFailed(format!("Invalid request body."))
        })?;
        let url = format!(
            "https://api.openai.com/v1/threads/{}/runs/{}/submit_tool_outputs",
            thread_id, run_id
        );

        let response = self
            .client
            .post(&url)
            .header("OpenAI-Beta", "assistants=v1")
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
            let response = response.json::<RetrieveRunResponse>().await.map_err(|e| {
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

    fn mock_response<T>(path: &str) -> Result<T, anyhow::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let json = fs::read_to_string(path).with_context(|| {
            format!(
                "Something went wrong reading the mock response file at '{}'",
                path
            )
        })?;
        let response = serde_json::from_str::<T>(json.as_str()).with_context(|| {
            format!(
                "Something went wrong reading the mock response file at '{}'",
                path
            )
        })?;

        Ok(response)
    }
}
