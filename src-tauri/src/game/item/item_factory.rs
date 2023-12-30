use futures::future;

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
    utils::random::Random,
};

use futures::StreamExt;

use super::{item_input::ItemInput, item_output::ItemOutput, Item};

pub struct ItemFactory<'a> {
    openai_client: &'a OpenAIClient,
    game_summary: &'a str,
    image_factory: &'a ImageFactory<'a>,
}

impl<'a> ItemFactory<'a> {
    pub fn new(
        openai_client: &'a OpenAIClient,
        game_summary: &'a str,
        image_factory: &'a ImageFactory,
    ) -> Self {
        Self {
            openai_client,
            game_summary,
            image_factory,
        }
    }

    pub async fn create_items(
        &self,
        item_list: Vec<String>,
    ) -> Result<Vec<Item>, OpenAIClientError> {
        let items_input = ItemInput::new(self.game_summary, item_list);
        let system_prompt = PromptBuilder::new()
            .add_prompt("./prompts/item_detail/main.txt")
            .add_example_input("./prompts/item_detail/example1_input.json")
            .add_example_output("./prompts/item_detail/example1_output.json")
            .add_example_input("./prompts/item_detail/example2_input.json")
            .add_example_output("./prompts/item_detail/example2_output.json")
            .build();
        let user_prompt = items_input.to_string();

        let text_response = self
            .openai_client
            .chat_completion_request(ChatCompletionRequest::new(
                system_prompt,
                user_prompt,
                ChatCompletionModel::Gpt_35_Turbo_1106,
            ))
            .await
            .expect("Failed to retrieve response from OpenAI")
            .get_content();

        let item_output = serde_json::from_str::<ItemOutput>(&text_response)
            .expect("Unable to deserialize output.");

        let items = async {
            let mut item_futures = Vec::new();
            for item in item_output.items {
                let item_future = async move {
                    let id = Random::generate_id();
                    let filepath = format!("items/{}.png", &id);
                    let image = self
                        .image_factory
                        .generate_image(
                            ImageGenerationRequest::new(
                                item.image,
                                ImageGenerationModel::Dall_E_2,
                                ImageGenerationSize::Size256x256,
                            ),
                            &filepath,
                        )
                        .await
                        .expect("Failed to generate image.");
                    Item::new(id, item.name, item.description, image)
                };
                item_futures.push(item_future)
            }

            let stream = futures::stream::iter(item_futures).buffered(3);
            stream.collect::<Vec<_>>().await
        };

        let items = items.await;
        Ok(items)
    }
}
