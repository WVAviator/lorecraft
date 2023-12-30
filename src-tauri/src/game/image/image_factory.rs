use std::{collections::HashMap, sync::RwLock};

use log::info;

use crate::{
    file_manager::FileManager,
    openai_client::{
        image_generation::{
            image_generation_model::ImageGenerationModel,
            image_generation_request::ImageGenerationRequest,
        },
        openai_client_error::OpenAIClientError,
        OpenAIClient,
    },
};

use super::Image;

pub struct ImageFactory<'a> {
    openai_client: &'a OpenAIClient,
    file_manager: &'a FileManager,
    game_id: &'a str,
    call_counts: RwLock<HashMap<ImageGenerationModel, u32>>,
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
            call_counts: RwLock::new(HashMap::new()),
        }
    }

    async fn rate_limit(&self, model: &ImageGenerationModel) {
        let call_count = {
            let mut call_counts = self.call_counts.write().unwrap();
            let call_count = call_counts.entry(model.clone()).or_insert(0);
            *call_count += 1;
            *call_count
        };

        if call_count > 5 {
            info!("Reached call limit for model: {:?}.", &model);
            info!("Sleeping for 60 seconds.");
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;

            {
                let mut call_counts = self.call_counts.write().unwrap();
                *call_counts.entry(model.clone()).or_insert(0) = 0;
            }
        }
    }

    pub async fn generate_image(
        &self,
        image_generation_request: ImageGenerationRequest,
        filepath: &str,
    ) -> Result<Image, OpenAIClientError> {
        self.rate_limit(&image_generation_request.model).await;

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
