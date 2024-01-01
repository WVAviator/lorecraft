use futures::StreamExt;

use crate::{
    game::image::image_factory::ImageFactory,
    openai_client::{
        chat_completion_model::ChatCompletionModel,
        chat_completion_request::ChatCompletionRequest,
        image_generation::{
            image_generation_model::ImageGenerationModel,
            image_generation_request::ImageGenerationRequest,
            image_generation_size::ImageGenerationSize,
        },
        openai_client_error::OpenAIClientError,
        OpenAIClient,
    },
    prompt_builder::PromptBuilder,
};

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

    pub async fn create(&self) -> Result<Narrative, OpenAIClientError> {
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
            .chat_completion_request(ChatCompletionRequest::new(
                system_prompt,
                user_prompt,
                ChatCompletionModel::Gpt3_5Turbo1106,
            ))
            .await
            .expect("Failed to get response from OpenAI API.")
            .get_content();

        let narrative = serde_json::from_str::<NarrativeOutput>(response_text.as_str())
            .expect("Failed to deserialize narrative.");

        let pages = async {
            let mut page_futures = Vec::new();
            for (index, page) in narrative.pages.iter().enumerate() {
                let page_future = async move {
                    let filepath = format!("narrative/page_{}.png", index);
                    let image = self
                        .image_factory
                        .try_generate_image(
                            ImageGenerationRequest::new(
                                page.image.clone(),
                                ImageGenerationModel::DallE3,
                                ImageGenerationSize::Size1792x1024,
                            ),
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
