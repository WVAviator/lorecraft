use crate::{
    game::{image::image_factory::ImageFactory, scene_detail::SceneDetail},
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
use log::info;

use super::{character_input::CharacterInput, character_output::CharacterOutput, Character};

pub struct CharacterFactory<'a> {
    game_summary: &'a str,
    openai_client: &'a OpenAIClient,
    image_factory: &'a ImageFactory<'a>,
}

impl<'a> CharacterFactory<'a> {
    pub fn new(
        game_summary: &'a str,
        openai_client: &'a OpenAIClient,
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
    ) -> Result<Vec<Character>, OpenAIClientError> {
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
                    let response_text = self
                        .openai_client
                        .chat_completion_request(ChatCompletionRequest::new(
                            system_prompt,
                            user_prompt,
                            ChatCompletionModel::Gpt_35_Turbo_1106,
                        ))
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
                        .generate_image(
                            ImageGenerationRequest::new(
                                character_output.image.to_string(),
                                ImageGenerationModel::Dall_E_2,
                                ImageGenerationSize::Size1024x1024,
                            ),
                            &filepath,
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
