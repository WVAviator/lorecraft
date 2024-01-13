use crate::{
    game::image::image_factory::ImageFactory, prompt_builder::PromptBuilder, utils::random::Random,
};

use anyhow::anyhow;
use futures::StreamExt;
use openai_lib::{
    chat_completion::{ChatCompletionClient, ChatCompletionRequest},
    image::{CreateImageRequest, ImageSize},
    model::{image_model::ImageModel, ChatModel},
    OpenAIClient,
};

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

    pub async fn create_items(&self, item_list: Vec<String>) -> Result<Vec<Item>, anyhow::Error> {
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
            .create_chat_completion(
                ChatCompletionRequest::builder()
                    .add_system_message(system_prompt)
                    .add_user_message(user_prompt)
                    .model(ChatModel::Gpt_35_Turbo_1106)
                    .build(),
            )
            .await
            .map_err(|e| anyhow!("Failed to create chat completion request: {:?}", e))?
            .get_content();

        let item_output = serde_json::from_str::<ItemOutput>(&text_response)
            .map_err(|e| anyhow!("API provided an invalid format for response: {:?}", e))?;

        let items = async {
            let mut item_futures = Vec::new();
            for item in item_output.items {
                let item_future = async move {
                    let id = Random::generate_id();
                    let filepath = format!("items/{}.png", &id);
                    let image = self
                        .image_factory
                        .try_generate_image(
                            CreateImageRequest::builder()
                                .prompt(&item.image)
                                .model(ImageModel::DallE2)
                                .size(ImageSize::Size256x256)
                                .build(),
                            &filepath,
                            3,
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
