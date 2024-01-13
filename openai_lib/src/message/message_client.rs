use crate::Error;

use super::{CreateMessageRequest, MessageObject};

#[trait_variant::make(MessageClient: Send)]
pub trait LocalMessageClient {
    async fn create_message(
        &self,
        request: CreateMessageRequest,
        thread_id: &str,
    ) -> Result<MessageObject, Error>;
}
