use anyhow::{anyhow, Context};
use log::{info, trace, warn};
use openai_lib::{
    chat_completion::{ChatCompletionClient, ChatCompletionRequest},
    model::ChatModel,
    OpenAIClient,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{config::content_setting::ContentSetting, file_manager::FileManager};

use super::game_metadata::GameMetadata;

/// The SelectionFactory should be used to provide prompts to OpenAI that consist of a list of
/// items to be selected. For example, choosing from a list of music themes based on a description.
pub struct SelectionFactory<'a> {
    openai_client: &'a OpenAIClient,
    file_manager: &'a FileManager,
    game_metadata: &'a GameMetadata,
}

/// A trait that can be implemented by any type that can be selected from a list of items. Required
/// for use in the SelectionFactory.
pub trait Selectable {
    fn select_from_response(response: &String, meta_path: &str) -> Result<Self, anyhow::Error>
    where
        Self: Sized;
}

impl<'a> SelectionFactory<'a> {
    pub fn new(
        openai_client: &'a OpenAIClient,
        file_manager: &'a FileManager,
        game_metadata: &'a GameMetadata,
    ) -> Self {
        SelectionFactory {
            openai_client,
            file_manager,
            game_metadata,
        }
    }

    /// Tries to select an item of type T from a provided list of items given messages as part of
    /// SelectionFactoryArgs. The type T item must implement Selectable to provide a function for
    /// translating between the response and the item.
    pub async fn try_create<T>(
        &self,
        factory_args: SelectionFactoryArgs,
    ) -> Result<T, anyhow::Error>
    where
        T: Serialize + DeserializeOwned + Selectable,
    {
        info!("Selecting for {}.", factory_args.name);

        let mut errors = Vec::new();

        for _ in 0..factory_args.max_attempts {
            match self.create(&factory_args).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    warn!(
                        "Failed to select {}, trying again. Error: {:?}",
                        factory_args.name, &e
                    );
                    errors.push(e);
                }
            }
        }

        Err(anyhow!(
            "Failed to select {}. Max attempts exceeded. Attempts returned the following errors: {:?}.",
            factory_args.name,
            errors
        ))
    }

    async fn create<T>(&self, factory_args: &SelectionFactoryArgs) -> Result<T, anyhow::Error>
    where
        T: Serialize + DeserializeOwned + Selectable,
    {
        let file_path = format!("{}/{}", self.game_metadata.game_id, factory_args.file_name);

        info!(
            "Checking for existing {} JSON file at {}",
            &factory_args.name, &file_path
        );

        match self.file_manager.file_exists(&file_path) {
            Ok(true) => {
                info!(
                    "Found existing {} JSON file. Loading...",
                    &factory_args.name
                );
                return self
                    .file_manager
                    .read_json::<T>(&file_path)
                    .context("Unable to read existing summary JSON file.");
            }
            _ => {
                info!("No existing {} found, selecting new...", &factory_args.name);
            }
        }

        let result = self.generate(factory_args).await?;
        let result = T::select_from_response(&result, &factory_args.meta_path)?;

        self.file_manager
            .write_json::<T>(&file_path, &result)
            .context("Unable to write to JSON file.")?;

        info!(
            "Generated {} and saved to '{}'.",
            &factory_args.name, &file_path
        );

        Ok(result)
    }

    async fn generate(&self, factory_args: &SelectionFactoryArgs) -> Result<String, anyhow::Error> {
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
                    .temperature(self.game_metadata.temperature_setting)
                    .build(),
            )
            .await
            .map_err(|e| anyhow!("Failed to create chat completion request: {}", e))?
            .get_content();

        trace!("{} response text: {}", factory_args.name, &response_text);

        Ok(response_text)
    }
}

pub struct SelectionFactoryArgs {
    name: String,
    system_message: String,
    user_message: String,
    file_name: String,
    max_attempts: u8,
    meta_path: String,
}

impl SelectionFactoryArgs {
    pub fn builder() -> SelectionFactoryArgsBuilder {
        SelectionFactoryArgsBuilder::new()
    }
}

pub struct SelectionFactoryArgsBuilder {
    name: Option<String>,
    system_message: Option<String>,
    user_message: Option<String>,
    file_name: Option<String>,
    max_attempts: Option<u8>,
    meta_path: Option<String>,
}

impl SelectionFactoryArgsBuilder {
    pub fn new() -> Self {
        SelectionFactoryArgsBuilder {
            name: None,
            system_message: None,
            user_message: None,
            file_name: None,
            max_attempts: None,
            meta_path: None,
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
    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = Some(file_name.into());
        self
    }
    #[allow(dead_code)]
    pub fn max_attempts(mut self, max_attempts: u8) -> Self {
        self.max_attempts = Some(max_attempts);
        self
    }
    pub fn meta_path(mut self, meta_path: impl Into<String>) -> Self {
        self.meta_path = Some(meta_path.into());
        self
    }
    pub fn build(self) -> SelectionFactoryArgs {
        SelectionFactoryArgs {
            name: self.name.unwrap_or(String::from("Unspecified")),
            system_message: self
                .system_message
                .unwrap_or(String::from("You are a helpful assistant")),
            user_message: self
                .user_message
                .unwrap_or(String::from("Pick a number between one and ten.")),
            file_name: self
                .file_name
                .unwrap_or(String::from("unspecified_file.json")),
            max_attempts: self.max_attempts.unwrap_or(3),
            meta_path: self.meta_path.unwrap_or(String::from("")),
        }
    }
}
