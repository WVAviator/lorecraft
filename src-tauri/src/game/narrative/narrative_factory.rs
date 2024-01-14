use anyhow::anyhow;
use futures::{StreamExt, TryStreamExt};
use log::info;
use openai_lib::{
    chat_completion::{ChatCompletionClient, ChatCompletionRequest},
    image::{CreateImageRequest, ImageQuality, ImageSize},
    model::{image_model::ImageModel, ChatModel},
    OpenAIClient,
};

use crate::{
    commands::create_new_game::create_new_game_request::CreateNewGameRequest,
    config::content_setting::ContentSetting, game::image::image_factory::ImageFactory,
    prompt_builder::PromptBuilder,
};

use super::{narrative_output::NarrativeOutput, Narrative, Page};

pub struct NarrativeFactory<'a> {
    openai_client: &'a OpenAIClient,
    summary: &'a str,
    image_factory: &'a ImageFactory<'a>,
    request: &'a CreateNewGameRequest,
}

impl<'a> NarrativeFactory<'a> {
    pub fn new(
        openai_client: &'a OpenAIClient,
        summary: &'a str,
        image_factory: &'a ImageFactory,
        request: &'a CreateNewGameRequest,
    ) -> Self {
        Self {
            openai_client,
            summary,
            image_factory,
            request,
        }
    }

    pub async fn create(&self) -> Result<Narrative, anyhow::Error> {
        let system_prompt = PromptBuilder::new()
            .add_prompt("./prompts/narrative/main.txt")
            .add_example_input("./prompts/narrative/example1_input.json")
            .add_example_output("./prompts/narrative/example1_output.json")
            .add_example_input("./prompts/narrative/example2_input.json")
            .add_example_output("./prompts/narrative/example2_output.json")
            .build();

        let user_prompt = self.summary.to_string();

        let model = match self.request.text_content_setting {
            Some(ContentSetting::Low) => ChatModel::Gpt_35_Turbo_1106,
            _ => ChatModel::Gpt_4_1106_Preview,
        };

        info!("Generating opening narrative for game.");

        let response_text = self
            .openai_client
            .create_chat_completion(
                ChatCompletionRequest::builder()
                    .add_system_message(system_prompt)
                    .add_user_message(user_prompt)
                    .model(model)
                    .temperature(self.request.get_temperature())
                    .json()
                    .build(),
            )
            .await
            .map_err(|e| anyhow!("Failed to create chat completion: {}", e))?
            .get_content();

        let narrative = serde_json::from_str::<NarrativeOutput>(response_text.as_str())
            .map_err(|e| anyhow!("Failed to deserialize narrative: {}", e))?;

        let pages = async {
            let mut page_futures = Vec::new();
            for (index, page) in narrative.pages.iter().enumerate() {
                let page_future = async move {
                    let filepath = format!("narrative/page_{}.png", index);

                    let quality = match self.request.image_content_setting {
                        Some(ContentSetting::High) => ImageQuality::HD,
                        _ => ImageQuality::Standard,
                    };

                    info!("Generating image for narrative page {}.", index);

                    let image = self
                        .image_factory
                        .try_generate_image(
                            CreateImageRequest::builder()
                                .prompt(&page.image)
                                .model(ImageModel::DallE3)
                                .size(ImageSize::Size1792x1024)
                                .quality(quality)
                                .build(),
                            &filepath,
                            3,
                        )
                        .await
                        .map_err(|e| anyhow!("Failed to generate narrative image: {}", e))?;

                    info!(
                        "Finished generating narrative page and image for index {}.",
                        index
                    );

                    Ok(Page::new(page.narrative.clone(), image)) as Result<Page, anyhow::Error>
                };
                page_futures.push(page_future);
            }

            let stream = futures::stream::iter(page_futures).buffered(3);
            stream.try_collect::<Vec<_>>().await
        };
        let pages = pages.await?;
        Ok(Narrative::new(pages))
    }
}
