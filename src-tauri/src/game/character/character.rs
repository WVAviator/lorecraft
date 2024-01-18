use futures::{StreamExt, TryStreamExt};
use log::info;
use openai_lib::{image::ImageQuality, model::image_model::ImageModel};
use serde::{Deserialize, Serialize};

use crate::{
    config::content_setting::ContentSetting,
    file_manager::FileManager,
    game::{
        character::character_input::CharacterInput,
        chat_completion_factory::{ChatCompletionFactory, ChatCompletionFactoryArgs},
        game_metadata::GameMetadata,
        image::{
            image_factory::{ImageFactory, ImageFactoryArgs},
            image_multiprocessor::ImageMultiprocessor,
            Image,
        },
        scene::Scene,
        summary::Summary,
    },
    prompt_builder::PromptBuilder,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    pub name: String,
    pub short_description: String,
    pub physical_description: String,
    pub speech: String,
    pub personality: String,
    pub backstory: String,
    pub thoughts: String,
    pub inventory: Vec<String>,
    pub image: Image,
}

impl Character {
    pub async fn create(
        summary: &Summary,
        scene_description: String,
        character_summary: String,
        factory: &ChatCompletionFactory<'_>,
    ) -> Result<Self, anyhow::Error> {
        let character_input =
            CharacterInput::new(&summary.summary, &scene_description, &character_summary);
        let system_prompt = PromptBuilder::new()
            .add_prompt("./prompts/character_detail/main.txt")
            .add_example_input("./prompts/character_detail/example1_input.json")
            .add_example_output("./prompts/character_detail/example1_output.json")
            .add_example_input("./prompts/character_detail/example2_input.json")
            .add_example_output("./prompts/character_detail/example2_output.json")
            .build();

        let user_prompt = character_input.to_string();

        let character_name: String = character_summary
            .split(":")
            .next()
            .unwrap_or(character_summary.as_str())
            .split("(")
            .next()
            .unwrap_or(character_summary.as_str())
            .trim()
            .to_string();

        info!("Creating character detail for {}", &character_name);

        // TODO: It's possible that two characters with the same name could be generated.
        let filepath = format!("tmp/characters/{}.json", &character_name);

        let character = factory
            .try_create(
                ChatCompletionFactoryArgs::builder()
                    .name(&character_name)
                    .system_message(system_prompt)
                    .user_message(user_prompt)
                    .file_name(filepath)
                    .before_save(Box::new(move |mut ch: Character| {
                        // Ensure that the LLM doesn't try to change the name, leading to filename
                        // mismatch
                        ch.name = character_name.clone();
                        ch
                    }))
                    .build(),
            )
            .await?;

        Ok(character)
    }

    pub async fn create_from_scenes(
        summary: &Summary,
        scenes: &Vec<Scene>,
        factory: &ChatCompletionFactory<'_>,
    ) -> Result<Vec<Character>, anyhow::Error> {
        let mut character_futures = Vec::new();

        info!("Generating character details for each scene.");

        for scene in scenes {
            for character_summary in &scene.characters {
                info!(
                    "Generating character profile from character entry: {}",
                    &character_summary
                );
                let character_future = Character::create(
                    &summary,
                    scene.narrative.clone(),
                    character_summary.clone(),
                    factory,
                );
                character_futures.push(character_future);
            }
        }

        let stream = futures::stream::iter(character_futures).buffered(3);
        let characters = stream.try_collect::<Vec<_>>().await?;

        Ok(characters)
    }

    pub async fn generate_image(
        &mut self,
        factory: &ImageFactory<'_>,
        game_metadata: &GameMetadata,
        file_manager: &FileManager,
    ) -> Result<(), anyhow::Error> {
        // TODO: It's possible that two characters with the same name could be generated.
        let filepath = format!("characters/{}.png", self.name);

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

        info!("Generated image for {}. Saving to JSON file.", &self.name);

        self.image = image.clone();

        file_manager
            .json_transaction::<Character, _>(
                format!(
                    "{}/tmp/characters/{}.json",
                    game_metadata.game_id, &self.name
                ),
                move |mut character| {
                    character.image = image;
                    character
                },
            )
            .await?;

        Ok(())
    }
}

impl ImageMultiprocessor for Vec<Character> {
    async fn generate_images(
        &mut self,
        factory: &ImageFactory<'_>,
        game_metadata: &GameMetadata,
        file_manager: &FileManager,
    ) -> Result<(), anyhow::Error> {
        info!("Generating images for each character.");

        let mut futures = Vec::new();

        for character in self {
            info!("Generating image for {}", &character.name);
            let future = character.generate_image(factory, game_metadata, file_manager);
            futures.push(future);
        }

        let stream = futures::stream::iter(futures).buffered(3);
        stream.try_collect::<Vec<_>>().await?;

        Ok(())
    }
}
