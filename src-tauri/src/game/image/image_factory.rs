use crate::{
    file_manager::FileManager,
    openai_client::{
        image_generation::image_generation_request::ImageGenerationRequest,
        openai_client_error::OpenAIClientError, OpenAIClient,
    },
};

use super::Image;

pub struct ImageFactory<'a> {
    openai_client: &'a OpenAIClient,
    file_manager: &'a FileManager,
    game_id: &'a str,
}

impl<'a> ImageFactory<'a> {
    pub fn new(
        openai_client: &'a OpenAIClient,
        file_manager: &'a FileManager,
        game_id: &'a str,
    ) -> Self {
        Self {
            openai_client,
            file_manager,
            game_id,
        }
    }

    pub async fn generate_image(
        &self,
        image_generation_request: ImageGenerationRequest,
        filepath: &str,
    ) -> Result<Image, OpenAIClientError> {
        let prompt = image_generation_request.user_prompt.clone();
        let response = self
            .openai_client
            .image_generation_request(image_generation_request)
            .await?;

        let alt = response.data[0]
            .revised_prompt
            .as_ref()
            .unwrap_or(&prompt)
            .clone();

        let base64_encoded = response.data[0].b64_json.split(",").last().unwrap();
        let image_data = base64::decode(base64_encoded).expect("Failed to decode base64 image.");

        let filepath = format!("{}/{}", self.game_id, filepath);

        let src = self
            .file_manager
            .write_bytes_to_file(&filepath, image_data)
            .expect("Failed to write image to file.");

        Ok(Image { src, alt })
    }
}
