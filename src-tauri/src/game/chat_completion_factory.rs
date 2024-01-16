use anyhow::{anyhow, Context};
use log::{info, trace};
use openai_lib::{
    chat_completion::{ChatCompletionClient, ChatCompletionRequest},
    model::ChatModel,
    OpenAIClient,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{config::content_setting::ContentSetting, file_manager::FileManager};

use super::game_metadata::GameMetadata;

pub struct ChatCompletionFactory<'a> {
    openai_client: &'a OpenAIClient,
    file_manager: &'a FileManager,
    game_metadata: &'a GameMetadata,
}

impl<'a> ChatCompletionFactory<'a> {
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

    pub async fn try_create<T>(
        &self,
        factory_args: ChatCompletionFactoryArgs,
    ) -> Result<T, anyhow::Error>
    where
        T: DeserializeOwned + Serialize,
    {
        info!("Creating {}.", factory_args.name);

        let mut errors = Vec::new();

        for _ in 0..factory_args.max_attempts {
            match self.create(&factory_args).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    info!(
                        "Failed to create {}, trying again. Error: {:?}",
                        factory_args.name, &e
                    );
                    errors.push(e);
                }
            }
        }

        Err(anyhow!(
            "Failed to create {}. Max attempts exceeded. Attempts returned the following errors: {:?}.",
            factory_args.name,
            errors
        ))
    }

    async fn create<T>(&self, factory_args: &ChatCompletionFactoryArgs) -> Result<T, anyhow::Error>
    where
        T: DeserializeOwned + Serialize,
    {
        let file_path = format!(
            "{}/tmp/{}",
            self.game_metadata.game_id, factory_args.file_name
        );

        info!("Checking for existing summary JSON file at {}", &file_path);

        match self.file_manager.file_exists(&file_path) {
            Ok(true) => {
                return self
                    .file_manager
                    .read_json::<T>(&file_path)
                    .context("Unable to read existing summary JSON file.");
            }
            _ => {
                info!("No existing summary found, generating new summary.");
            }
        }

        let result = self.generate::<T>(factory_args).await?;

        self.file_manager
            .write_json::<T>(&file_path, &result)
            .context("Unable to write to JSON file.")?;

        info!(
            "Generated {} and saved to '{}'.",
            &factory_args.name, &file_path
        );

        Ok(result)
    }

    async fn generate<T>(
        &self,
        factory_args: &ChatCompletionFactoryArgs,
    ) -> Result<T, anyhow::Error>
    where
        T: DeserializeOwned,
    {
        let model = match self.game_metadata.text_content_setting {
            ContentSetting::Low => ChatModel::Gpt_35_Turbo_1106,
            _ => ChatModel::Gpt_4_1106_Preview,
        };

        let response_text = self
            .openai_client
            .create_chat_completion(
                ChatCompletionRequest::builder()
                    .add_system_message(&factory_args.system_message)
                    .add_user_message(&factory_args.user_message)
                    .model(model)
                    .json()
                    .temperature(self.game_metadata.temperature_setting)
                    .build(),
            )
            .await
            .map_err(|e| anyhow!("Failed to create chat completion request: {}", e))?
            .get_content();

        trace!("{} response text: {}", factory_args.name, &response_text);

        let result = serde_json::from_str::<T>(response_text.as_str())
            .map_err(|e| anyhow!("Failed to deserialize {}: {}", factory_args.name, e))?;

        Ok(result)
    }
}

pub struct ChatCompletionFactoryArgs {
    name: String,
    system_message: String,
    user_message: String,
    max_attempts: u8,
    file_name: String,
}

impl ChatCompletionFactoryArgs {
    pub fn builder() -> ChatCompletionFactoryArgsBuilder {
        ChatCompletionFactoryArgsBuilder::new()
    }
}

pub struct ChatCompletionFactoryArgsBuilder {
    name: Option<String>,
    system_message: Option<String>,
    user_message: Option<String>,
    max_attempts: u8,
    file_name: Option<String>,
}

impl ChatCompletionFactoryArgsBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            system_message: None,
            user_message: None,
            max_attempts: 3,
            file_name: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn system_message(mut self, system_message: impl Into<String>) -> Self {
        self.system_message = Some(system_message.into());
        self
    }

    pub fn user_message(mut self, user_message: impl Into<String>) -> Self {
        self.user_message = Some(user_message.into());
        self
    }

    pub fn max_attempts(mut self, max_attempts: u8) -> Self {
        self.max_attempts = max_attempts;
        self
    }

    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = Some(file_name.into());
        self
    }

    pub fn build(mut self) -> ChatCompletionFactoryArgs {
        self.name.get_or_insert(String::from("Missing"));
        self.system_message.get_or_insert(String::new());
        self.user_message.get_or_insert(String::new());
        self.file_name.get_or_insert(String::from("missing.json"));

        ChatCompletionFactoryArgs {
            name: self.name.unwrap(),
            system_message: self.system_message.unwrap(),
            user_message: self.user_message.unwrap(),
            max_attempts: self.max_attempts,
            file_name: self.file_name.unwrap(),
        }
    }
}
