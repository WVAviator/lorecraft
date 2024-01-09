use self::{
    chat_completion_object::ChatCompletionObject, chat_completion_request::ChatCompletionRequest,
};

pub mod chat_completion_object;
pub mod chat_completion_request;
pub mod log_probability;
pub mod usage_statistics;

pub trait ChatCompletionClient {
    async fn chat_completion_request(
        &self,
        chat_completion_request: ChatCompletionRequest,
    ) -> Result<ChatCompletionObject, reqwest::Error>;
}
