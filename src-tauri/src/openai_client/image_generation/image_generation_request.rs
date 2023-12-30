use log::warn;
use serde_json::json;

use super::{
    image_generation_model::ImageGenerationModel, image_generation_size::ImageGenerationSize,
};

#[derive(Debug, Clone)]
pub struct ImageGenerationRequest {
    pub user_prompt: String,
    pub model: ImageGenerationModel,
    pub size: ImageGenerationSize,
}

impl ImageGenerationRequest {
    pub fn new(
        user_prompt: String,
        model: ImageGenerationModel,
        size: ImageGenerationSize,
    ) -> Self {
        let model = match (&size, &model) {
            (
                ImageGenerationSize::Size256x256 | ImageGenerationSize::Size512x512,
                ImageGenerationModel::Dall_E_3,
            ) => {
                warn!(
                    "Image size {} not supported by model {}, using dall-e-2 instead",
                    size.to_string(),
                    model.to_string()
                );
                ImageGenerationModel::Dall_E_2
            }
            (
                ImageGenerationSize::Size1024x1792 | ImageGenerationSize::Size1792x1024,
                ImageGenerationModel::Dall_E_2,
            ) => {
                warn!(
                    "Image size {} not supported by model {}, using dall-e-3 instead",
                    size.to_string(),
                    model.to_string()
                );
                ImageGenerationModel::Dall_E_3
            }
            _ => model,
        };

        ImageGenerationRequest {
            user_prompt,
            model,
            size,
        }
    }

    pub fn to_request_body(self) -> String {
        json!({
          "model": self.model.to_string(),
          "prompt": self.user_prompt,
          "n": 1,
          "size": self.size.to_string(),
          "response_format": "b64_json"
        })
        .to_string()
    }
}
