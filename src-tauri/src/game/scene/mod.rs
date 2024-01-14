use crate::{
    commands::create_new_game::create_new_game_request::CreateNewGameRequest,
    config::content_setting::ContentSetting, utils::random::Random,
};

use super::{
    image::{image_factory::ImageFactory, Image},
    scene_detail::SceneDetail,
};

use openai_lib::{
    image::{CreateImageRequest, ImageQuality, ImageSize},
    model::image_model::ImageModel,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub id: String,
    pub name: String,
    pub narrative: String,
    pub metadata: String,
    pub characters: Vec<String>,
    pub items: Vec<String>,
    pub image: Image,
}

impl Scene {
    pub async fn from_scene_detail(
        scene_detail: &SceneDetail,
        image_factory: &ImageFactory<'_>,
        request: &CreateNewGameRequest,
    ) -> Result<Self, anyhow::Error> {
        let id = Random::generate_id();

        let name = scene_detail.name.clone();
        let narrative = scene_detail.narrative.clone();
        let metadata = scene_detail.metadata.clone();

        let filepath = format!("scenes/{}.png", id);

        let (model, quality) = match request.image_content_setting {
            Some(ContentSetting::Low) => (ImageModel::DallE2, ImageQuality::Standard),
            Some(ContentSetting::High) => (ImageModel::DallE3, ImageQuality::HD),
            _ => (ImageModel::DallE3, ImageQuality::Standard),
        };

        let image = image_factory
            .try_generate_image(
                CreateImageRequest::builder()
                    .prompt(&scene_detail.image)
                    .model(model)
                    .quality(quality)
                    .size(ImageSize::Size1024x1024)
                    .build(),
                &filepath,
                3,
            )
            .await?;

        Ok(Scene {
            id,
            name,
            narrative,
            metadata,
            characters: scene_detail.characters.clone(),
            items: scene_detail.items.clone(),
            image,
        })
    }
}
