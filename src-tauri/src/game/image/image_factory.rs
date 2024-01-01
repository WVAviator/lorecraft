use std::{collections::VecDeque, sync::RwLock, time::SystemTime};

use base64::{engine::general_purpose, Engine as _};
use log::{debug, error, info, trace};

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
    ts_deque: RwLock<VecDeque<i64>>,
    style: String,
}

impl<'a> ImageFactory<'a> {
    pub fn new(
        openai_client: &'a OpenAIClient,
        file_manager: &'a FileManager,
        game_id: &'a str,
        style: String,
    ) -> Self {
        Self {
            openai_client,
            file_manager,
            game_id,
            ts_deque: RwLock::new(VecDeque::new()),
            style,
        }
    }

    async fn rate_limit(&self) {
        loop {
            let is_limited = {
                let mut ts_deque = self.ts_deque.write().unwrap();

                let now = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as i64;

                while let Some(front) = ts_deque.front() {
                    if now - front > 60000 {
                        ts_deque.pop_front();
                    } else {
                        break;
                    }
                }

                ts_deque.len() >= 5
            };

            if !is_limited {
                break;
            }

            info!("Throttling API requests for image generation.");
            debug!("Sleeping for 15 seconds.");
            tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
        }

        {
            let mut ts_deque = self.ts_deque.write().unwrap();
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64;
            ts_deque.push_back(now);
        }
    }

    pub async fn try_generate_image(
        &self,
        image_generation_request: ImageGenerationRequest,
        filepath: &str,
        max_attempts: u32,
    ) -> Result<Image, OpenAIClientError> {
        let mut attempts = 0;
        while attempts < max_attempts {
            match self
                .generate_image(image_generation_request.clone(), filepath)
                .await
            {
                Ok(image) => return Ok(image),
                Err(e) => {
                    error!("Image API request failed with reason: {:?}", e);
                    trace!("Request:\n{:?}", &image_generation_request);
                    info!("Retrying...")
                }
            }
            attempts += 1;
        }

        Err(OpenAIClientError::MaxAttemptsExceeded(format!(
            "Max attempts exceeded to generate image for request:\n{:?}",
            &image_generation_request
        )))
    }

    async fn generate_image(
        &self,
        image_generation_request: ImageGenerationRequest,
        filepath: &str,
    ) -> Result<Image, OpenAIClientError> {
        self.rate_limit().await;

        let prompt = format!("{}\n{}", &image_generation_request.user_prompt, &self.style);
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
        let image_data = general_purpose::STANDARD
            .decode(base64_encoded)
            .expect("Failed to decode base64 image.");

        let filepath = format!("{}/{}", self.game_id, filepath);

        let src = self
            .file_manager
            .write_bytes_to_file(&filepath, image_data)
            .expect("Failed to write image to file.");

        Ok(Image { src, alt })
    }
}
