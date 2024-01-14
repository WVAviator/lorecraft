use crate::{
    commands::create_new_game::create_new_game_request::CreateNewGameRequest,
    config::content_setting::ContentSetting,
    game::{image::image_factory::ImageFactory, scene_detail::SceneDetail},
    prompt_builder::PromptBuilder,
    utils::random::Random,
};

use anyhow::anyhow;
use futures::{StreamExt, TryStreamExt};
use log::info;
use openai_lib::{
    chat_completion::{ChatCompletionClient, ChatCompletionRequest},
    image::{CreateImageRequest, ImageQuality, ImageSize},
    model::{image_model::ImageModel, ChatModel},
    OpenAIClient,
};

use super::{character_input::CharacterInput, character_output::CharacterOutput, Character};

pub struct CharacterFactory<'a> {
    game_summary: &'a str,
    openai_client: &'a OpenAIClient,
    image_factory: &'a ImageFactory<'a>,
    request: &'a CreateNewGameRequest,
}

impl<'a> CharacterFactory<'a> {
    pub fn new(
        openai_client: &'a OpenAIClient,
        game_summary: &'a str,
        image_factory: &'a ImageFactory<'a>,
        request: &'a CreateNewGameRequest,
    ) -> Self {
        Self {
            game_summary,
            openai_client,
            image_factory,
            request,
        }
    }

    pub async fn from_scene_detail(
        &self,
        scene_detail: &SceneDetail,
    ) -> Result<Vec<Character>, anyhow::Error> {
        info!("Generating characters from scene: {}", &scene_detail.name);

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

                let model = match self.request.text_content_setting {
                    Some(ContentSetting::Low) => ChatModel::Gpt_35_Turbo_1106,
                    _ => ChatModel::Gpt_4_1106_Preview,
                };

                let character_future = async move {
                    let response_text = self
                        .openai_client
                        .create_chat_completion(
                            ChatCompletionRequest::builder()
                                .model(model)
                                .add_system_message(system_prompt)
                                .add_user_message(user_prompt)
                                .temperature(self.request.get_temperature())
                                .json()
                                .build(),
                        )
                        .await
                        .map_err(|e| anyhow!("Failed to create chat completion request: {}", e))?
                        .get_content();

                    let character_output = serde_json::from_str::<CharacterOutput>(&response_text)
                        .map_err(|e| anyhow!("Failed to deserialize character output: {}", e))?;

                    info!("Generating image for character {}", &character_output.name);

                    let id = Random::generate_id();
                    let filepath = format!("characters/{}.png", &id);

                    let (model, quality) = match self.request.image_content_setting {
                        Some(ContentSetting::Low) => (ImageModel::DallE2, ImageQuality::Standard),
                        Some(ContentSetting::High) => (ImageModel::DallE3, ImageQuality::HD),
                        _ => (ImageModel::DallE3, ImageQuality::Standard),
                    };

                    let image = self
                        .image_factory
                        .try_generate_image(
                            CreateImageRequest::builder()
                                .prompt(&character_output.image)
                                .model(model)
                                .size(ImageSize::Size1024x1024)
                                .quality(quality)
                                .build(),
                            &filepath,
                            3,
                        )
                        .await
                        .map_err(|e| anyhow!("Failed to generate image: {}", e))?;

                    info!(
                        "Finshed image and profile for character '{}'",
                        &character_output.name
                    );

                    Ok(Character::new(character_output, id, image))
                        as Result<Character, anyhow::Error>
                };

                character_futures.push(character_future);
            }

            let stream = futures::stream::iter(character_futures).buffered(3);
            stream.try_collect::<Vec<_>>().await
        };

        let characters = characters.await?;
        Ok(characters)
    }
}
