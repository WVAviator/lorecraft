use crate::Error;

use super::{
    create_moderation_request::CreateModerationRequest, moderation_object::ModerationObject,
};

#[trait_variant::make(ModerationClient: Send)]
pub trait LocalModerationClient {
    async fn create_moderation(
        &self,
        request: CreateModerationRequest,
    ) -> Result<ModerationObject, Error>;

    async fn moderate(&self, input: &str) -> Result<(), Error>;
}
