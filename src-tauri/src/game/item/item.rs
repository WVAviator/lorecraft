use futures::{StreamExt, TryStreamExt};
use log::info;
use openai_lib::{
    image::{ImageQuality, ImageSize},
    model::image_model::ImageModel,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    config::content_setting::ContentSetting,
    file_manager::FileManager,
    game::{
        character::Character,
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

use super::item_input::ItemInput;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub image: Image,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ItemsResult {
    items: Vec<Item>,
}

impl Item {
    pub async fn create(
        summary: &Summary,
        item_list: Vec<String>,
        factory: &ChatCompletionFactory<'_>,
    ) -> Result<Vec<Item>, anyhow::Error> {
        let items_input = ItemInput::new(&summary.summary, item_list);
        let system_prompt = PromptBuilder::new()
            .add_prompt("./prompts/item_detail/main.txt")
            .add_example_input("./prompts/item_detail/example1_input.json")
            .add_example_output("./prompts/item_detail/example1_output.json")
            .add_example_input("./prompts/item_detail/example2_input.json")
            .add_example_output("./prompts/item_detail/example2_output.json")
            .build();
        let user_prompt = items_input.to_string();

        info!("Prepared system and user messages for generating item details.");

        let result = factory
            .try_create::<ItemsResult>(
                ChatCompletionFactoryArgs::builder()
                    .name("Items")
                    .system_message(system_prompt)
                    .user_message(user_prompt)
                    .file_name("tmp/items.json")
                    .build(),
            )
            .await?;

        let items = result.items;

        Ok(items)
    }

    pub async fn create_from_scenes_and_chars(
        summary: &Summary,
        scenes: &Vec<Scene>,
        characters: &Vec<Character>,
        factory: &ChatCompletionFactory<'_>,
    ) -> Result<Vec<Item>, anyhow::Error> {
        let scene_item_list = scenes
            .iter()
            .map(|scene| scene.items.clone())
            .flatten()
            .collect::<Vec<String>>();
        let character_item_list = characters
            .iter()
            .map(|character| character.inventory.clone())
            .flatten()
            .collect::<Vec<String>>();
        let mut item_list = scene_item_list
            .iter()
            .chain(character_item_list.iter())
            .cloned()
            .collect::<Vec<String>>();

        info!("Created item list from scenes and characters.");

        item_list.dedup(); // No need to generate data for the same items twice
        item_list.sort();

        info!("Removed duplicates and sorted list. Generating details...");

        let items = Item::create(summary, item_list, factory).await?;

        Ok(items)
    }

    pub async fn generate_image(
        &mut self,
        factory: &ImageFactory<'_>,
        game_metadata: &GameMetadata,
        file_manager: &FileManager,
    ) -> Result<(), anyhow::Error> {
        let filepath = format!("items/{}.png", self.name);

        let (model, size) = match game_metadata.image_content_setting {
            ContentSetting::High => (ImageModel::DallE3, ImageSize::Size1024x1024),
            _ => (ImageModel::DallE2, ImageSize::Size512x512),
        };

        info!("Generating image for item {}...", self.name);

        let image = factory
            .try_create(
                &self.image,
                ImageFactoryArgs::builder()
                    .model(model)
                    .size(size)
                    .quality(ImageQuality::Standard)
                    .filepath(filepath)
                    .build(),
            )
            .await?;

        info!(
            "Generated image for item {}. Saving to items JSON file.",
            self.name
        );

        self.image = image.clone();

        let name = self.name.clone();

        file_manager
            .json_transaction::<ItemsResult, _>(
                format!("{}/tmp/items.json", game_metadata.game_id),
                move |mut items_result| {
                    if let Some(item) = items_result.items.iter_mut().find(|item| item.name == name)
                    {
                        item.image = image;
                    }
                    items_result
                },
            )
            .await?;

        Ok(())
    }
}

impl ImageMultiprocessor for Vec<Item> {
    async fn generate_images(
        &mut self,
        factory: &ImageFactory<'_>,
        game_metadata: &GameMetadata,
        file_manager: &FileManager,
    ) -> Result<(), anyhow::Error> {
        let mut futures = Vec::new();
        for item in self {
            let future = item.generate_image(factory, game_metadata, file_manager);
            futures.push(future);
        }
        let stream = futures::stream::iter(futures).buffered(5);
        stream.try_collect::<Vec<_>>().await?;

        Ok(())
    }
}
