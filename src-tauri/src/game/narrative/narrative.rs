use futures::{StreamExt, TryStreamExt};
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
        summary::Summary,
    },
    prompt_builder::PromptBuilder,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Narrative {
    pages: Vec<Page>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Page {
    pub narrative: String,
    pub image: Image,
}

impl Narrative {
    pub async fn create(
        summary: &Summary,
        factory: &ChatCompletionFactory<'_>,
    ) -> Result<Self, anyhow::Error> {
        let system_prompt = PromptBuilder::new()
            .add_prompt("./prompts/narrative/main.txt")
            .add_example_input("./prompts/narrative/example1_input.json")
            .add_example_output("./prompts/narrative/example1_output.json")
            .add_example_input("./prompts/narrative/example2_input.json")
            .add_example_output("./prompts/narrative/example2_output.json")
            .build();

        let user_prompt = summary.summary.clone();

        info!("Prepared system and user messages for narrative.");

        let narrative = factory
            .try_create::<Narrative>(
                ChatCompletionFactoryArgs::builder()
                    .name("Narrative")
                    .system_message(system_prompt)
                    .user_message(user_prompt)
                    .file_name("tmp/narrative.json")
                    .build(),
            )
            .await?;

        Ok(narrative)
    }

    pub async fn generate_images(
        &mut self,
        factory: &ImageFactory<'_>,
        game_metadata: &GameMetadata,
        file_manager: &FileManager,
    ) -> Result<(), anyhow::Error> {
        let mut futures = Vec::new();

        info!("Generating images for narrative pages.");

        for (index, page) in self.pages.iter().enumerate() {
            let page = page.clone();
            let future = async move {
                info!("Generating image for narrative page: {}", index);

                if let Image::Created { .. } = page.image {
                    info!("Image already created and saved - skipping.");
                    return Ok(page);
                }

                let (model, quality) = match game_metadata.image_content_setting {
                    ContentSetting::High => (ImageModel::DallE3, ImageQuality::HD),
                    _ => (ImageModel::DallE3, ImageQuality::Standard),
                };

                let filepath = format!("narrative/page_{}.png", index);

                let image = factory
                    .try_create(
                        &page.image,
                        ImageFactoryArgs::builder()
                            .size(ImageSize::Size1792x1024)
                            .model(model)
                            .quality(quality)
                            .filepath(filepath)
                            .build(),
                    )
                    .await?;

                info!("Image generated for narrative page {}. Saving new iamge data to narrative JSON file.", index);

                file_manager
                    .json_transaction::<Narrative, _>(
                        format!("{}/tmp/narrative.json", game_metadata.game_id),
                        move |mut narrative| {
                            if let Some(page) = narrative.pages.get_mut(index) {
                                page.image = image.clone();
                            }
                            narrative
                        },
                    )
                    .await?;

                Ok(page) as Result<Page, anyhow::Error>
            };
            futures.push(future);
        }

        let stream = futures::stream::iter(futures).buffered(3);
        let pages = stream.try_collect::<Vec<_>>().await?;

        info!("All narrative page images generated and saved.");

        self.pages = pages;

        Ok(())
    }
}
