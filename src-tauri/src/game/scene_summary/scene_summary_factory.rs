use anyhow::{anyhow, Context};
use log::info;
use openai_lib::{
    chat_completion::{ChatCompletionClient, ChatCompletionRequest},
    model::ChatModel,
    OpenAIClient,
};

use crate::{
    config::content_setting::ContentSetting,
    file_manager::FileManager,
    game::{game_metadata::GameMetadata, image::image_factory::ImageFactory, summary::Summary},
    prompt_builder::PromptBuilder,
};

use super::{scene_summary_input::SceneSummaryInput, SceneSummary};

pub struct SceneSummaryFactory<'a> {
    openai_client: &'a OpenAIClient,
    file_manager: &'a FileManager,
    game_metadata: &'a GameMetadata,
    image_factory: &'a ImageFactory<'a>,
}

impl<'a> SceneSummaryFactory<'a> {
    pub fn new(
        openai_client: &'a OpenAIClient,
        file_manager: &'a FileManager,
        game_metadata: &'a GameMetadata,
        image_factory: &'a ImageFactory,
    ) -> Self {
        Self {
            openai_client,
            file_manager,
            game_metadata,
            image_factory,
        }
    }

    pub async fn try_create(
        &self,
        summary: &Summary,
        max_attempts: u8,
    ) -> Result<SceneSummary, anyhow::Error> {
        info!("Creating scene summaries.");

        let mut errors = Vec::new();

        for _ in 0..max_attempts {
            match self.create(summary).await {
                Ok(scene_summary) => return Ok(scene_summary),
                Err(e) => {
                    info!(
                        "Failed to create scene summaries, trying again. Error: {:?}",
                        &e
                    );
                    errors.push(e);
                }
            }
        }

        Err(anyhow!(
            "Failed to create summaries. Max attempts exceeded. Attempts returned the following errors: {:?}.",
            errors
        ))
    }

    async fn create(&self, summary: &Summary) -> Result<SceneSummary, anyhow::Error> {
        let scene_summary_path = format!("{}/tmp/scene_summary.json", self.game_metadata.game_id);

        info!(
            "Checking for existing scene summary JSON file at {}",
            &scene_summary_path
        );

        match self.file_manager.file_exists(&scene_summary_path) {
            Ok(true) => {
                return self
                    .file_manager
                    .read_json::<SceneSummary>(&scene_summary_path)
                    .context("Unable to read existing scene summary JSON file.");
            }
            _ => {
                info!(
                    "No existing scene summary JSON file found at '{}'. Creating new scene summary.",
                    &scene_summary_path
                );
            }
        }

        let scene_summary = self.generate_scene_summary(summary).await?;

        self.file_manager
            .write_json::<SceneSummary>(&scene_summary_path, &scene_summary)
            .context("Unable to write scene summary JSON file.")?;

        Ok(scene_summary)
    }

    async fn generate_scene_summary(
        &self,
        summary: &Summary,
    ) -> Result<SceneSummary, anyhow::Error> {
        let input = SceneSummaryInput::new(
            summary.summary.to_string(),
            summary.win_condition.to_string(),
        );

        let system_prompt = PromptBuilder::new()
            .add_prompt("./prompts/scene_summary/main.txt")
            .add_example_input("./prompts/scene_summary/example1_input.json")
            .add_example_output("./prompts/scene_summary/example1_output.json")
            .add_example_input("./prompts/scene_summary/example2_input.json")
            .add_example_output("./prompts/scene_summary/example2_output.json")
            .build();

        let user_prompt = input.to_string();

        let model = match self.game_metadata.text_content_setting {
            ContentSetting::Low => ChatModel::Gpt_35_Turbo_1106,
            _ => ChatModel::Gpt_4_1106_Preview,
        };

        let response_text = self
            .openai_client
            .create_chat_completion(
                ChatCompletionRequest::builder()
                    .add_system_message(system_prompt)
                    .add_user_message(user_prompt)
                    .temperature(self.game_metadata.temperature_setting)
                    .model(model)
                    .json()
                    .build(),
            )
            .await
            .map_err(|e| anyhow!("Failed to create chat completion request: {}", e))?
            .get_content();

        let scene_summary = serde_json::from_str::<SceneSummary>(response_text.as_str())
            .map_err(|e| anyhow!("Failed to deserialize scene summary: {}", e))?;

        Ok(scene_summary)
    }
}
