use futures::{StreamExt, TryStreamExt};
use log::info;
use openai_lib::{image::ImageQuality, model::image_model::ImageModel};
use serde::{Deserialize, Serialize};

use crate::{
    config::content_setting::ContentSetting,
    file_manager::FileManager,
    game::{
        chat_completion_factory::{ChatCompletionFactory, ChatCompletionFactoryArgs},
        game_metadata::GameMetadata,
        image::{
            image_factory::{ImageFactory, ImageFactoryArgs},
            image_multiprocessor::ImageMultiprocessor,
            Image,
        },
        scene_summary::{SceneSummary, SummarizedScene},
        summary::Summary,
    },
    prompt_builder::PromptBuilder,
};

use super::scene_input::SceneInput;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Scene {
    pub name: String,
    pub narrative: String,
    pub metadata: String,
    pub image: Image,
    pub characters: Vec<String>,
    pub items: Vec<String>,
}

impl Scene {
    pub async fn create_all(
        summary: &Summary,
        scene_summary: &SceneSummary,
        factory: &ChatCompletionFactory<'_>,
    ) -> Result<Vec<Scene>, anyhow::Error> {
        let mut futures = Vec::new();

        info!("Creating scenes...");

        for summarized_scene in &scene_summary.scenes {
            let future = Scene::create(summary, summarized_scene, factory);
            futures.push(future);
        }

        let stream = futures::stream::iter(futures).buffered(3);
        stream.try_collect::<Vec<_>>().await
    }

    pub async fn create(
        summary: &Summary,
        summarized_scene: &SummarizedScene,
        factory: &ChatCompletionFactory<'_>,
    ) -> Result<Self, anyhow::Error> {
        let scene_detail_input = SceneInput::new(&summary.summary, summarized_scene);

        let system_message = PromptBuilder::new()
            .add_prompt("./prompts/scene_detail/main.txt")
            .add_example_input("./prompts/scene_detail/example1_input.json")
            .add_example_output("./prompts/scene_detail/example1_output.json")
            .add_example_input("./prompts/scene_detail/example2_input.json")
            .add_example_output("./prompts/scene_detail/example2_output.json")
            .build();

        let user_message = serde_json::to_string(&scene_detail_input).unwrap();

        info!(
            "Prepared system and user message for generating scene '{}'.",
            &summarized_scene.name
        );

        let scene_name = summarized_scene.name.clone();
        let name = format!("{} Scene Detail", &scene_name);
        let file_name = format!("tmp/scenes/{}.json", &scene_name);

        factory
            .try_create(
                ChatCompletionFactoryArgs::builder()
                    .name(&name)
                    .system_message(system_message)
                    .user_message(user_message)
                    .file_name(file_name)
                    .before_save(Box::new(move |mut scene: Scene| {
                        // Ensure that the LLM doesn't try to change the name, leading to filename
                        // mismatch
                        scene.name = scene_name.clone();
                        scene
                    }))
                    .build(),
            )
            .await
    }

    async fn generate_image(
        &mut self,
        factory: &ImageFactory<'_>,
        game_metadata: &GameMetadata,
        file_manager: &FileManager,
    ) -> Result<(), anyhow::Error> {
        let filepath = format!("scenes/{}.png", self.name);

        let (model, quality) = match game_metadata.image_content_setting {
            ContentSetting::High => (ImageModel::DallE3, ImageQuality::HD),
            ContentSetting::Low => (ImageModel::DallE2, ImageQuality::Standard),
            _ => (ImageModel::DallE3, ImageQuality::Standard),
        };

        let image = factory
            .try_create(
                &self.image,
                ImageFactoryArgs::builder()
                    .model(model)
                    .quality(quality)
                    .filepath(filepath)
                    .build(),
            )
            .await?;

        info!(
            "Created image for scene {}. Saving to JSON file.",
            &self.name
        );

        self.image = image.clone();

        file_manager
            .json_transaction::<Scene, _>(
                format!("{}/tmp/scenes/{}.json", game_metadata.game_id, self.name),
                move |mut scene_detail| {
                    scene_detail.image = image;
                    scene_detail
                },
            )
            .await?;

        Ok(())
    }
}

impl ImageMultiprocessor for Vec<Scene> {
    async fn generate_images(
        &mut self,
        factory: &ImageFactory<'_>,
        game_metadata: &GameMetadata,
        file_manager: &FileManager,
    ) -> Result<(), anyhow::Error> {
        let mut futures = Vec::new();
        for scene_detail in self {
            let future = scene_detail.generate_image(factory, game_metadata, file_manager);
            futures.push(future);
        }
        let stream = futures::stream::iter(futures).buffered(3);
        stream.try_collect::<Vec<_>>().await?;

        Ok(())
    }
}
