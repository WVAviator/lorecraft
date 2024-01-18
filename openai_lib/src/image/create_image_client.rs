use crate::Error;

use super::{create_image_request::CreateImageRequest, create_image_response::CreateImageResponse};

#[trait_variant::make(CreateImageClient: Send)]
pub trait LocalCreateImageClient {
    async fn create_image(&self, request: CreateImageRequest)
        -> Result<CreateImageResponse, Error>;
}
