use crate::common::error::Error;

use self::{
    chat_completion_object::ChatCompletionObject, chat_completion_request::ChatCompletionRequest,
};

pub mod chat_completion_message;
pub mod chat_completion_object;
pub mod chat_completion_request;
pub mod log_probability;
pub mod usage_statistics;

pub trait ChatCompletionClient {
    async fn create_chat_completion(
        &self,
        chat_completion_request: ChatCompletionRequest,
    ) -> Result<ChatCompletionObject, Error>;
}
