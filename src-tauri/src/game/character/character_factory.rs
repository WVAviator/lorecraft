use crate::{
    game::{image::image_factory::ImageFactory, scene_detail::SceneDetail},
    prompt_builder::PromptBuilder,
    utils::random::Random,
};

use futures::StreamExt;
use log::info;
use openai_lib::{
    chat_completion::{ChatCompletionClient, ChatCompletionRequest},
    image::{CreateImageRequest, ImageSize},
    model::{image_model::ImageModel, ChatModel},
    OpenAIClient,
};

use super::{character_input::CharacterInput, character_output::CharacterOutput, Character};

pub struct CharacterFactory<'a> {
    game_summary: &'a str,
    openai_client: &'a OpenAIClient,
    image_factory: &'a ImageFactory<'a>,
}

impl<'a> CharacterFactory<'a> {
    pub fn new(
        openai_client: &'a OpenAIClient,
        game_summary: &'a str,
        image_factory: &'a ImageFactory<'a>,
    ) -> Self {
        Self {
            game_summary,
            openai_client,
            image_factory,
        }
    }

    pub async fn from_scene_detail(
        &self,
        scene_detail: &SceneDetail,
    ) -> Result<Vec<Character>, anyhow::Error> {
        let scene_description = scene_detail.narrative.clone();
        let characters = async {
            let mut character_futures = Vec::new();
            for character_summary in &scene_detail.characters {
                let character_input =
                    CharacterInput::new(self.game_summary, &scene_description, character_summary);
                let system_prompt = PromptBuilder::new()
                    .add_prompt("./prompts/character_detail/main.txt")
                    .add_example_input("./prompts/character_detail/example1_input.json")
                    .add_example_output("./prompts/character_detail/example1_output.json")
                    .add_example_input("./prompts/character_detail/example2_input.json")
                    .add_example_output("./prompts/character_detail/example2_output.json")
                    .build();

                let user_prompt = character_input.to_string();

                info!(
                    "Creating character detail for {}",
                    &character_summary.split(":").next().unwrap()
                );

                let character_future = async move {
                    let request = ChatCompletionRequest::builder()
                        .model(ChatModel::Gpt_35_Turbo_1106)
                        .add_system_message(system_prompt)
                        .add_user_message(user_prompt)
                        .build();

                    let response_text = self
                        .openai_client
                        .create_chat_completion(request)
                        .await
                        .expect("Failed to get response from OpenAI API")
                        .get_content();

                    let character_output = serde_json::from_str::<CharacterOutput>(&response_text)
                        .expect("Failed to deserialize character output.");

                    info!("Generating image for character {}", &character_output.name);

                    let id = Random::generate_id();
                    let filepath = format!("characters/{}.png", &id);
                    let image = self
                        .image_factory
                        .try_generate_image(
                            CreateImageRequest::builder()
                                .prompt(&character_output.image)
                                .model(ImageModel::DallE3)
                                .size(ImageSize::Size1024x1024)
                                .build(),
                            &filepath,
                            3,
                        )
                        .await
                        .expect("Failed to generate image.");

                    Character::new(character_output, id, image)
                };

                character_futures.push(character_future);
            }

            let stream = futures::stream::iter(character_futures).buffered(3);
            stream.collect::<Vec<_>>().await
        };

        let characters = characters.await;
        Ok(characters)
    }
}
