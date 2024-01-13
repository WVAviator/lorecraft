use crate::assistant::{
    AssistantClient, AssistantObject, CreateAssistantRequest, DeleteAssistantResponse,
};
use crate::chat_completion::{ChatCompletionClient, ChatCompletionObject, ChatCompletionRequest};
use crate::client_config::ClientConfig;
use crate::image::create_image_client::CreateImageClient;
use crate::image::create_image_request::CreateImageRequest;
use crate::image::create_image_response::CreateImageResponse;
use crate::message::{CreateMessageRequest, MessageClient, MessageObject};
use crate::rate_limit::RateLimiter;
use crate::thread::{CreateThreadRequest, DeleteThreadResponse, ThreadClient, ThreadObject};
use crate::Error;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    ClientBuilder,
};
use serde::de::DeserializeOwned;

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
            return Err(Error::ResponseFailure(response.status()));
        }
    }
}

impl ChatCompletionClient for OpenAIClient {
    async fn create_chat_completion(
        &self,
        chat_completion_request: ChatCompletionRequest,
    ) -> Result<ChatCompletionObject, Error> {
        let body = chat_completion_request.to_json_body();
        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .body(body?)
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
}
