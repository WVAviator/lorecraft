use crate::assistant::{
    AssistantClient, AssistantObject, CreateAssistantRequest, DeleteAssistantResponse,
};
use crate::chat_completion::{ChatCompletionClient, ChatCompletionObject, ChatCompletionRequest};
use crate::client_config::ClientConfig;
use crate::image::create_image_client::CreateImageClient;
use crate::image::create_image_request::CreateImageRequest;
use crate::image::create_image_response::CreateImageResponse;
use crate::message::list_messages_response::ListMessagesResponse;
use crate::message::{CreateMessageRequest, ListMessagesRequest, MessageClient, MessageObject};
use crate::moderation::{CreateModerationRequest, ModerationClient, ModerationObject};
use crate::rate_limit::RateLimiter;
use crate::run::{CreateRunRequest, RunClient, RunObject, SubmitToolOutputsRequest};
use crate::thread::{CreateThreadRequest, DeleteThreadResponse, ThreadClient, ThreadObject};
use crate::Error;
use log::error;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    ClientBuilder,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct OpenAIClient {
    client: reqwest::Client,
    image_rate_limiter: RateLimiter,
}

impl OpenAIClient {
    pub fn new(config: ClientConfig) -> Result<Self, Error> {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", config.api_key))
                .map_err(|e| Error::ConfigurationFailure(e.into()))?,
        );

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .map_err(|e| Error::ConfigurationFailure(e.into()))?;

        let image_rate_limiter = RateLimiter::new(std::time::Duration::from_secs(60), 5);

        Ok(Self {
            client,
            image_rate_limiter,
        })
    }

    pub async fn verify_connection(&self) -> Result<(), Error> {
        let response = self
            .client
            .get("https://api.openai.com/v1/models")
            .send()
            .await
            .map_err(|e| Error::RequestFailure(e.into()))?;

        self.handle_response::<Value>(response).await?;

        Ok(())
    }

    async fn handle_response<T: DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T, Error> {
        if response.status().is_success() {
            let response = response
                .json::<T>()
                .await
                .map_err(|e| Error::DeserializationFailure(e.into()))?;

            return Ok(response);
        } else {
            let status_code = response.status();

            error!(
                "Bad response from OpenAI:\n{}\n\n{}",
                response.status(),
                response.text().await.unwrap()
            );

            return Err(Error::ResponseFailure(status_code));
        }
    }
}

impl ChatCompletionClient for OpenAIClient {
    async fn create_chat_completion(
        &self,
        chat_completion_request: ChatCompletionRequest,
    ) -> Result<ChatCompletionObject, Error> {
        let body = chat_completion_request.to_json_body()?;
        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .body(body)
            .send()
            .await
            .map_err(|e| Error::RequestFailure(e.into()))?;

        self.handle_response::<ChatCompletionObject>(response).await
    }
}

impl CreateImageClient for OpenAIClient {
    async fn create_image(
        &self,
        request: CreateImageRequest,
    ) -> Result<CreateImageResponse, Error> {
        let body = request.to_json_body()?;

        self.image_rate_limiter
            .permit()
            .await
            .map_err(|e| Error::RateLimitFailure(e.into()))?;

        let response = self
            .client
            .post("https://api.openai.com/v1/images/generations")
            .body(body)
            .send()
            .await
            .map_err(|e| Error::RequestFailure(e.into()))?;

        self.handle_response::<CreateImageResponse>(response).await
    }
}

impl AssistantClient for OpenAIClient {
    async fn create_assistant(
        &self,
        request: CreateAssistantRequest,
    ) -> Result<AssistantObject, Error> {
        let body = request.to_json_body()?;

        let response = self
            .client
            .post("https://api.openai.com/v1/assistants")
            .body(body)
            .header("OpenAI-Beta", "assistants=v1")
            .send()
            .await
            .map_err(|e| Error::RequestFailure(e.into()))?;

        self.handle_response::<AssistantObject>(response).await
    }

    async fn delete_assistant(&self, assistant_id: &str) -> Result<DeleteAssistantResponse, Error> {
        let url = format!("https://api.openai.com/v1/assistants/{}", assistant_id);

        let response = self
            .client
            .delete(url)
            .header("OpenAI-Beta", "assistants=v1")
            .send()
            .await
            .map_err(|e| Error::RequestFailure(e.into()))?;

        self.handle_response::<DeleteAssistantResponse>(response)
            .await
    }
}

impl ThreadClient for OpenAIClient {
    async fn create_thread(&self, request: CreateThreadRequest) -> Result<ThreadObject, Error> {
        let body = request.to_json_body()?;

        let response = self
            .client
            .post("https://api.openai.com/v1/threads")
            .header("OpenAI-Beta", "assistants=v1")
            .body(body)
            .send()
            .await
            .map_err(|e| Error::RequestFailure(e.into()))?;

        self.handle_response::<ThreadObject>(response).await
    }

    async fn delete_thread(&self, thread_id: &str) -> Result<DeleteThreadResponse, Error> {
        let url = format!("https://api.openai.com/v1/threads/{}", thread_id);

        let response = self
            .client
            .delete(url)
            .header("OpenAI-Beta", "assistants=v1")
            .send()
            .await
            .map_err(|e| Error::RequestFailure(e.into()))?;

        self.handle_response::<DeleteThreadResponse>(response).await
    }
}

impl MessageClient for OpenAIClient {
    async fn create_message(
        &self,
        request: CreateMessageRequest,
        thread_id: &str,
    ) -> Result<MessageObject, Error> {
        let body = request.to_json_body()?;
        let url = format!("https://api.openai.com/v1/threads/{}/messages", thread_id);

        let response = self
            .client
            .post(url)
            .body(body)
            .header("OpenAI-Beta", "assistants=v1")
            .send()
            .await
            .map_err(|e| Error::RequestFailure(e.into()))?;

        self.handle_response::<MessageObject>(response).await
    }

    async fn list_messages(
        &self,
        request: ListMessagesRequest,
        thread_id: &str,
    ) -> Result<ListMessagesResponse, Error> {
        let url = request.build_url(format!(
            "https://api.openai.com/v1/threads/{}/messages",
            thread_id
        ))?;

        let response = self
            .client
            .get(url)
            .header("OpenAI-Beta", "assistants=v1")
            .send()
            .await
            .map_err(|e| Error::RequestFailure(e.into()))?;

        self.handle_response::<ListMessagesResponse>(response).await
    }
}

impl RunClient for OpenAIClient {
    async fn create_run(
        &self,
        request: CreateRunRequest,
        thread_id: &str,
    ) -> Result<RunObject, Error> {
        let body = request.to_json_body()?;
        let url = format!("https://api.openai.com/v1/threads/{}/runs", thread_id);

        let response = self
            .client
            .post(url)
            .header("OpenAI-Beta", "assistants=v1")
            .body(body)
            .send()
            .await
            .map_err(|e| Error::RequestFailure(e.into()))?;

        self.handle_response::<RunObject>(response).await
    }

    async fn retrieve_run(&self, thread_id: &str, run_id: &str) -> Result<RunObject, Error> {
        let url = format!(
            "https://api.openai.com/v1/threads/{}/runs/{}",
            thread_id, run_id
        );

        let response = self
            .client
            .get(url)
            .header("OpenAI-Beta", "assistants=v1")
            .send()
            .await
            .map_err(|e| Error::RequestFailure(e.into()))?;

        self.handle_response::<RunObject>(response).await
    }

    async fn submit_tool_outputs(
        &self,
        request: SubmitToolOutputsRequest,
        thread_id: &str,
        run_id: &str,
    ) -> Result<RunObject, Error> {
        let body = request.to_json_body()?;
        let url = format!(
            "https://api.openai.com/v1/threads/{}/runs/{}/submit_tool_outputs",
            thread_id, run_id
        );

        let response = self
            .client
            .post(url)
            .header("OpenAI-Beta", "assistants=v1")
            .body(body)
            .send()
            .await
            .map_err(|e| Error::RequestFailure(e.into()))?;

        self.handle_response::<RunObject>(response).await
    }
}

impl ModerationClient for OpenAIClient {
    async fn create_moderation(
        &self,
        create_moderation_request: CreateModerationRequest,
    ) -> Result<ModerationObject, Error> {
        let body = create_moderation_request.to_json_body()?;

        let response = self
            .client
            .post("https://api.openai.com/v1/moderations")
            .body(body)
            .send()
            .await
            .map_err(|e| Error::RequestFailure(e.into()))?;

        self.handle_response::<ModerationObject>(response).await
    }

    async fn moderate(&self, input: &str) -> Result<(), Error> {
        let moderation_object = self
            .create_moderation(
                CreateModerationRequest::builder()
                    .input(input.to_string())
                    .build(),
            )
            .await?;
        match moderation_object.is_flagged() {
            true => Err(Error::ContentPolicyViolation(
                moderation_object.failure_reasons(),
            )),
            false => Ok(()),
        }
    }
}
