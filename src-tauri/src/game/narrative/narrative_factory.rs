use anyhow::anyhow;
use futures::StreamExt;
use openai_lib::{
    chat_completion::{ChatCompletionClient, ChatCompletionRequest},
    image::{CreateImageRequest, ImageQuality, ImageSize},
    model::{image_model::ImageModel, ChatModel},
    OpenAIClient,
};

use crate::{game::image::image_factory::ImageFactory, prompt_builder::PromptBuilder};

use super::{narrative_output::NarrativeOutput, Narrative, Page};

pub struct NarrativeFactory<'a> {
    openai_client: &'a OpenAIClient,
    summary: &'a str,
    image_factory: &'a ImageFactory<'a>,
}

impl<'a> NarrativeFactory<'a> {
    pub fn new(
        openai_client: &'a OpenAIClient,
        summary: &'a str,
        image_factory: &'a ImageFactory,
    ) -> Self {
        Self {
            openai_client,
            summary,
            image_factory,
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

        let response_text = self
            .openai_client
            .create_chat_completion(
                ChatCompletionRequest::builder()
                    .add_system_message(system_prompt)
                    .add_user_message(user_prompt)
                    .model(ChatModel::Gpt_35_Turbo_1106)
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
                    let image = self
                        .image_factory
                        .try_generate_image(
                            CreateImageRequest::builder()
                                .prompt(&page.image)
                                .model(ImageModel::DallE3)
                                .size(ImageSize::Size1792x1024)
                                .quality(ImageQuality::HD)
                                .build(),
                            &filepath,
                            3,
                        )
                        .await
                        .expect("Unable to generate image.");
                    Page::new(page.narrative.clone(), image)
                };
                page_futures.push(page_future);
            }

            let stream = futures::stream::iter(page_futures).buffered(3);
            stream.collect::<Vec<_>>().await
        };
        let pages = pages.await;
        Ok(Narrative::new(pages))
    }
}
