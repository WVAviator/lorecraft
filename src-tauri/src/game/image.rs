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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    pub src: String,
    pub alt: String,
}

impl Image {
    pub async fn from_image_prompt(
        prompt: &str,
        openai_client: &OpenAIClient,
        filepath: &str,
        model: ImageGenerationModel,
        size: ImageGenerationSize,
    ) -> Result<Self, OpenAIClientError> {
        let request = ImageGenerationRequest::new(prompt.to_string(), model, size);
        let response = openai_client
            .image_generation_request(request)
            .await
            .expect("Failed to get response from OpenAI API.");

        let alt = response.data[0].revised_prompt.clone();
        let base64_encoded = response.data[0].b64_json.split(",").last().unwrap();
        let image_data = base64::decode(base64_encoded).expect("Failed to decode base64 image.");

        let src = FileManager::new()
            .write_bytes_to_file(filepath, image_data)
            .expect("Failed to write image to file.");

        Ok(Image { src, alt })
    }
}
