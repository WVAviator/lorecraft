use log::info;
use serde::{Deserialize, Serialize};

use crate::{
    file_manager::FileManager,
    openai_client::{
        image_generation::{
            image_generation_model::ImageGenerationModel,
            image_generation_request::ImageGenerationRequest,
            image_generation_size::ImageGenerationSize,
        },
        openai_client_error::OpenAIClientError,
        OpenAIClient,
    },
};

pub mod image_factory;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    pub src: String,
    pub alt: String,
}
