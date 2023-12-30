use core::time;
use std::{
    collections::{BinaryHeap, HashMap, VecDeque},
    sync::RwLock,
    time::SystemTime,
};

use log::{debug, info};

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
    calls: RwLock<HashMap<ImageGenerationModel, VecDeque<i64>>>,
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
            calls: RwLock::new(HashMap::new()),
            style,
        }
    }

    async fn rate_limit(&self, model: &ImageGenerationModel) {
        loop {
            let is_limited = {
                let mut calls = self.calls.write().unwrap();

                let now = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as i64;

                let ts_deque = calls.entry(model.clone()).or_insert(VecDeque::new());

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

            info!("Throttling API requests for model: {:?}.", model.clone());
            debug!("Sleeping for 10 seconds.");
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }

        {
            let mut calls = self.calls.write().unwrap();
            let ts_deque = calls.entry(model.clone()).or_insert(VecDeque::new());
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64;
            ts_deque.push_back(now);
        }
    }

    pub async fn generate_image(
        &self,
        image_generation_request: ImageGenerationRequest,
        filepath: &str,
    ) -> Result<Image, OpenAIClientError> {
        self.rate_limit(&image_generation_request.model).await;

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
        let image_data = base64::decode(base64_encoded).expect("Failed to decode base64 image.");

        let filepath = format!("{}/{}", self.game_id, filepath);

        let src = self
            .file_manager
            .write_bytes_to_file(&filepath, image_data)
            .expect("Failed to write image to file.");

        Ok(Image { src, alt })
    }
}
