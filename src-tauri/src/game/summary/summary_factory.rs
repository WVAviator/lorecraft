use anyhow::{anyhow, Context};
use log::{info, trace};
use openai_lib::{
    chat_completion::{ChatCompletionClient, ChatCompletionRequest},
    model::ChatModel,
    OpenAIClient,
};

use crate::{
    config::content_setting::ContentSetting, file_manager::FileManager,
    game::game_metadata::GameMetadata, prompt_builder::PromptBuilder,
};

use super::Summary;

pub struct SummaryFactory<'a> {
    openai_client: &'a OpenAIClient,
    file_manager: &'a FileManager,
    game_metadata: &'a GameMetadata,
}

impl<'a> SummaryFactory<'a> {
    pub fn new(
        openai_client: &'a OpenAIClient,
        file_manager: &'a FileManager,
        game_metadata: &'a GameMetadata,
    ) -> Self {
        Self {
            openai_client,
            file_manager,
            game_metadata,
        }
    }

    pub async fn try_create(&self, max_attempts: u8) -> Result<Summary, anyhow::Error> {
        info!("Creating game summary.");

        let mut errors = Vec::new();

        for _ in 0..max_attempts {
            match self.create().await {
                Ok(summary) => return Ok(summary),
                Err(e) => {
                    info!("Failed to create summary, trying again. Error: {:?}", &e);
                    errors.push(e);
                }
            }
        }

        Err(anyhow!(
            "Failed to create summary. Max attempts exceeded. Attempts returned the following errors: {:?}.",
            errors
        ))
    }

    async fn create(&self) -> Result<Summary, anyhow::Error> {
        let summary_path = format!("{}/tmp/summary.json", &self.game_metadata.game_id);

        info!(
            "Checking for existing summary JSON file at {}",
            &summary_path
        );

        match self.file_manager.file_exists(&summary_path) {
            Ok(true) => {
                return self
                    .file_manager
                    .read_json::<Summary>(&summary_path)
                    .context("Unable to read existing summary JSON file.");
            }
            _ => {
                info!("No existing summary found, generating new summary.");
            }
        }

        let summary = self.generate_summary().await?;

        self.file_manager
            .write_json::<Summary>(&summary_path, &summary)
            .context("Unable to write summary JSON file.")?;

        info!("Generated summary for new game '{}'.", &summary.name);

        Ok(summary)
    }

    async fn generate_summary(&self) -> Result<Summary, anyhow::Error> {
        let system_prompt = PromptBuilder::new()
            .add_prompt("./prompts/summary/main.txt")
            .add_plain_text("Example Input: Make a game about mystical forests and ancient ruins")
            .add_example_output("./prompts/summary/example1.json")
            .add_plain_text("Example Input: I want to wake up on an abandoned spaceship infested with alien life")
            .add_example_output("./prompts/summary/example2.json")
            .build();

        let user_prompt = String::from(self.game_metadata.prompt.to_string());

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
                    .model(model)
                    .json()
                    .temperature(self.game_metadata.temperature_setting)
                    .build(),
            )
            .await
            .map_err(|e| anyhow!("Failed to create chat completion request: {}", e))?
            .get_content();

        trace!("Summary response text: {}", &response_text);

        let summary = serde_json::from_str::<Summary>(response_text.as_str())
            .map_err(|e| anyhow!("Failed to deserialize summary: {}", e))?;

        Ok(summary)
    }
}
