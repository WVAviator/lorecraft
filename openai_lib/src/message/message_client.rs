use crate::Error;

use super::{
    list_messages_response::ListMessagesResponse, CreateMessageRequest, ListMessagesRequest,
    MessageObject,
};

#[trait_variant::make(MessageClient: Send)]
pub trait LocalMessageClient {
    async fn create_message(
        &self,
        request: CreateMessageRequest,
        thread_id: &str,
    ) -> Result<MessageObject, Error>;

    async fn list_messages(
        &self,
        request: ListMessagesRequest,
        thread_id: &str,
    ) -> Result<ListMessagesResponse, Error>;
}
