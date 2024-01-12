use crate::chat_completion::{ChatCompletionClient, ChatCompletionObject, ChatCompletionRequest};
use crate::client_config::ClientConfig;
use crate::Error;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    ClientBuilder,
};
use serde::de::DeserializeOwned;

pub struct OpenAIClient {
    client: reqwest::Client,
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

        Ok(Self { client })
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
            .post("https://api.openai.com/v1/engines/davinci/completions")
            .body(body?)
            .send()
            .await
            .map_err(|e| Error::RequestFailure(e.into()))?;

        self.handle_response::<ChatCompletionObject>(response).await
    }
}
