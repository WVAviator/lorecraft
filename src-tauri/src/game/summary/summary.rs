use log::info;
use openai_lib::{
    image::{ImageQuality, ImageSize},
    model::image_model::ImageModel,
};
use serde::{Deserialize, Serialize};

use crate::{
    config::content_setting::ContentSetting,
    file_manager::FileManager,
    game::{
        chat_completion_factory::{ChatCompletionFactory, ChatCompletionFactoryArgs},
        game_metadata::GameMetadata,
        image::{
            image_factory::{ImageFactory, ImageFactoryArgs},
            Image,
        },
    },
    prompt_builder::PromptBuilder,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Summary {
    pub name: String,
    pub description: String,
    pub art_style: String,
    pub art_theme: String,
    pub cover_art: Image,
    pub summary: String,
    pub win_condition: String,
}

impl Summary {
    pub async fn create(
        factory: &ChatCompletionFactory<'_>,
        user_message: &str,
    ) -> Result<Self, anyhow::Error> {
        let system_message = PromptBuilder::new()
            .add_prompt("./prompts/summary/main.txt")
            .add_plain_text("Example Input: Make a game about mystical forests and ancient ruins")
            .add_example_output("./prompts/summary/example1.json")
            .add_plain_text("Example Input: I want to wake up on an abandoned spaceship infested with alien life")
            .add_example_output("./prompts/summary/example2.json")
            .build();

        let user_message = String::from(user_message);

        info!("Prepared system and user messages for summary.");

        factory
            .try_create(
                ChatCompletionFactoryArgs::builder()
                    .name("Summary")
                    .system_message(system_message)
                    .user_message(user_message)
                    .file_name("tmp/summary.json")
                    .build(),
            )
            .await
    }

    pub async fn generate_images(
        &mut self,
        image_factory: &ImageFactory<'_>,
        game_metadata: &GameMetadata,
        file_manager: &FileManager,
    ) -> Result<(), anyhow::Error> {
        let (model, quality) = match game_metadata.image_content_setting {
            ContentSetting::High => (ImageModel::DallE3, ImageQuality::HD),
            _ => (ImageModel::DallE3, ImageQuality::Standard),
        };

        info!("Generating cover art for game.");

        let cover_art = image_factory
            .try_create(
                &self.cover_art,
                ImageFactoryArgs::builder()
                    .model(model)
                    .quality(quality)
                    .size(ImageSize::Size1792x1024)
                    .filepath("summary/cover_art.png")
                    .build(),
            )
            .await?;

        self.cover_art = cover_art.clone();

        info!("Populating existing JSON data for summary with new cover art image.");

        file_manager
            .json_transaction::<Self, _>(
                format!("{}/tmp/summary.json", game_metadata.game_id),
                |mut summary| {
                    summary.cover_art = cover_art;
                    summary
                },
            )
            .await?;

        Ok(())
    }
}
