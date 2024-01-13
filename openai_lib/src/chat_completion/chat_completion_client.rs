use crate::Error;

use super::{ChatCompletionObject, ChatCompletionRequest};

#[trait_variant::make(ChatCompletionClient: Send)]
pub trait LocalChatCompletionClient {
    async fn create_chat_completion(
        &self,
        chat_completion_request: ChatCompletionRequest,
    ) -> Result<ChatCompletionObject, Error>;
}
