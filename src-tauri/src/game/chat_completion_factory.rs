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

/// A factory that can produce Chat Completions based on the game settings and configuration.
/// It can be used to pass any prompt to OpenAI and get an object of any shape back from the
/// assistant.
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

    /// Process a chat completion request with the provided ChatCompletionFactoryArgs and attempts
    /// to parse an object of type T from the response.
    ///
    /// Takes a ChatCompletionFactoryArgs argument, which can be constructed from a builder.
    pub async fn try_create<T>(
        &self,
        factory_args: ChatCompletionFactoryArgs<T>,
    ) -> Result<T, anyhow::Error>
    where
        T: DeserializeOwned + Serialize,
    {
        info!("Creating chat completion for {}.", factory_args.name);

        let mut errors = Vec::new();

        for _ in 0..factory_args.max_attempts {
            match self.create(&factory_args).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    warn!(
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

    async fn create<T>(
        &self,
        factory_args: &ChatCompletionFactoryArgs<T>,
    ) -> Result<T, anyhow::Error>
    where
        T: DeserializeOwned + Serialize,
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
                info!(
                    "No existing {} found, generating new...",
                    &factory_args.name
                );
            }
        }

        let result = self.generate::<T>(factory_args).await?;

        let result = (factory_args.before_save)(result);

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
        factory_args: &ChatCompletionFactoryArgs<T>,
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

        info!("Generated and parsed new {}", factory_args.name);

        Ok(result)
    }
}

/// Used as an argument to the ChatCompletionFactory try_create method.
/// ```
/// let args = ChatCompletionFactoryArgs::builder()
///     .name("HelloWorld")
///     .system_message("You are a helpful assistant.")
///     .user_message("Generate the following JSON...")
///     .max_attempts(3),
///     .file_name("hello-world.json")
///     .build();
/// ```
///
pub struct ChatCompletionFactoryArgs<T> {
    name: String,
    system_message: String,
    user_message: String,
    max_attempts: u8,
    file_name: String,
    before_save: Box<dyn Fn(T) -> T + Send + Sync + 'static>,
}

impl<T> ChatCompletionFactoryArgs<T> {
    pub fn builder() -> ChatCompletionFactoryArgsBuilder<T> {
        ChatCompletionFactoryArgsBuilder::new()
    }
}

pub struct ChatCompletionFactoryArgsBuilder<T> {
    name: Option<String>,
    system_message: Option<String>,
    user_message: Option<String>,
    max_attempts: u8,
    file_name: Option<String>,
    before_save: Option<Box<dyn Fn(T) -> T + Send + Sync + 'static>>,
}

impl<T> ChatCompletionFactoryArgsBuilder<T> {
    pub fn new() -> Self {
        Self {
            name: None,
            system_message: None,
            user_message: None,
            max_attempts: 3,
            file_name: None,
            before_save: None,
        }
    }

    /// The name of this generated item. This will be used strictly for logging and debugging
    /// purposes.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// The initial system message that the model should receive.
    pub fn system_message(mut self, system_message: impl Into<String>) -> Self {
        self.system_message = Some(system_message.into());
        self
    }

    /// The initial user message that should follow the system message.
    pub fn user_message(mut self, user_message: impl Into<String>) -> Self {
        self.user_message = Some(user_message.into());
        self
    }

    /// The maximum times that the API should be called to attempt to generate the desired object.
    /// Any errors in sending the request or in reading and deserializing the response will count
    /// as an attempt towards this total. Default is 3.
    pub fn max_attempts(mut self, max_attempts: u8) -> Self {
        self.max_attempts = max_attempts;
        self
    }

    /// The file name to store the result of deserializing the response. If this filename already
    /// exists, that value will be used instead - saving an API call and allows resuming the
    /// generation process.
    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = Some(file_name.into());
        self
    }

    /// A function that will be called before the result is saved to file. This can be used to
    /// modify the JSON data before it is saved.
    pub fn before_save(mut self, before_save: Box<dyn Fn(T) -> T + Send + Sync + 'static>) -> Self {
        self.before_save = Some(before_save);
        self
    }

    pub fn build(mut self) -> ChatCompletionFactoryArgs<T> {
        self.name.get_or_insert(String::from("Missing"));
        self.system_message.get_or_insert(String::new());
        self.user_message.get_or_insert(String::new());
        self.file_name.get_or_insert(String::from("missing.json"));
        self.before_save.get_or_insert(Box::new(|result| result));

        ChatCompletionFactoryArgs {
            name: self.name.unwrap(),
            system_message: self.system_message.unwrap(),
            user_message: self.user_message.unwrap(),
            max_attempts: self.max_attempts,
            file_name: self.file_name.unwrap(),
            before_save: self.before_save.unwrap(),
        }
    }
}
