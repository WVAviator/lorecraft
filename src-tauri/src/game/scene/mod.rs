use crate::{
    openai_client::{
        image_generation::{
            image_generation_model::ImageGenerationModel,
            image_generation_request::ImageGenerationRequest,
            image_generation_size::ImageGenerationSize,
        },
        openai_client_error::OpenAIClientError,
    },
    utils::random::Random,
};

use super::{
    image::{image_factory::ImageFactory, Image},
    scene_detail::SceneDetail,
};

pub struct Scene {
    pub id: String,
    pub name: String,
    pub narrative: String,
    pub metadata: String,
    pub image: Image,
}

impl Scene {
    pub async fn from_scene_detail(
        scene_detail: SceneDetail,
        image_factory: &ImageFactory<'_>,
    ) -> Result<Self, OpenAIClientError> {
        let id = Random::generate_id();

        let name = scene_detail.name;
        let narrative = scene_detail.narrative;
        let metadata = scene_detail.metadata;

        let image_generation_request = ImageGenerationRequest::new(
            scene_detail.image,
            ImageGenerationModel::Dall_E_2,
            ImageGenerationSize::Size1024x1024,
        );

        let filepath = format!("{}/image.png", id);

        let image = image_factory
            .generate_image(image_generation_request, &filepath)
            .await?;

        Ok(Scene {
            id,
            name,
            narrative,
            metadata,
            image,
        })
    }
}
