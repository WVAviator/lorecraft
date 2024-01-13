use crate::Error;

use super::{
    assistant_object::AssistantObject, create_assistant_request::CreateAssistantRequest,
    delete_assistant_response::DeleteAssistantResponse,
};

#[trait_variant::make(AssistantClient: Send)]
pub trait LocalAssistantClient {
    async fn create_assistant(
        &self,
        request: CreateAssistantRequest,
    ) -> Result<AssistantObject, Error>;
    async fn delete_assistant(&self, assistant_id: &str) -> Result<DeleteAssistantResponse, Error>;
}
