use anyhow::{anyhow, Context};
use log::{info, warn};
use openai_lib::{
    audio::{AudioClient, CreateSpeechRequest, TTSModel, TTSResponseFormat, TTSVoice},
    OpenAIClient,
};

use crate::{file_manager::FileManager, game::game_metadata::GameMetadata};

use super::Audio;

pub struct AudioFactory<'a> {
    openai_client: &'a OpenAIClient,
    file_manager: &'a FileManager,
    game_metadata: &'a GameMetadata,
}

impl<'a> AudioFactory<'a> {
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

    pub async fn try_create(&self, factory_args: AudioFactoryArgs) -> Result<Audio, anyhow::Error> {
        info!("Creating TTS audio for {}.", factory_args.name);

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

    async fn create(&self, factory_args: &AudioFactoryArgs) -> Result<Audio, anyhow::Error> {
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
                    .read_json::<Audio>(&file_path)
                    .context("Unable to read existing summary JSON file.");
            }
            _ => {
                info!("No existing {} found, selecting new...", &factory_args.name);
            }
        }

        let result = self.generate(factory_args).await?;

        self.file_manager
            .write_json::<Audio>(&file_path, &result)
            .context("Unable to write to JSON file.")?;

        info!(
            "Generated {} and saved to '{}'.",
            &factory_args.name, &file_path
        );

        Ok(result)
    }

    async fn generate(&self, factory_args: &AudioFactoryArgs) -> Result<Audio, anyhow::Error> {
        let bytes = self
            .openai_client
            .create_speech(
                CreateSpeechRequest::builder()
                    .input(factory_args.text.clone())
                    .voice(factory_args.voice.clone())
                    .model(TTSModel::TTS1_HD)
                    .speed(factory_args.speed)
                    .response_format(TTSResponseFormat::Mp3)
                    .build(),
            )
            .await
            .map_err(|e| anyhow!("Failed to create TTS: {}", e))?;

        info!(
            "Received raw audio data for '{}' from OpenAI.",
            factory_args.name
        );

        let filepath = format!("{}/{}", self.game_metadata.game_id, factory_args.audio_file);

        let src = self
            .file_manager
            .write_bytes_to_file(&filepath, bytes.into())
            .map_err(|e| anyhow!("Failed to write audio to file: {:?}", e))?;

        info!(
            "Audio '{}' created and saved to disk.",
            factory_args.audio_file
        );

        Ok(Audio {
            src,
            caption: factory_args.text.clone(),
        })
    }
}

pub struct AudioFactoryArgs {
    name: String,
    text: String,
    file_name: String,
    audio_file: String,
    voice: TTSVoice,
    max_attempts: u8,
    speed: f32,
}

impl AudioFactoryArgs {
    pub fn builder() -> AudioFactoryArgsBuilder {
        AudioFactoryArgsBuilder::new()
    }
}

pub struct AudioFactoryArgsBuilder {
    name: Option<String>,
    text: Option<String>,
    file_name: Option<String>,
    audio_file: Option<String>,
    voice: Option<TTSVoice>,
    max_attempts: Option<u8>,
    speed: Option<f32>,
}

impl AudioFactoryArgsBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            text: None,
            file_name: None,
            audio_file: None,
            voice: None,
            max_attempts: None,
            speed: None,
        }
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = Some(file_name.into());
        self
    }
    pub fn voice(mut self, voice: TTSVoice) -> Self {
        self.voice = Some(voice);
        self
    }
    #[allow(dead_code)]
    pub fn max_attempts(mut self, max_attempts: u8) -> Self {
        self.max_attempts = Some(max_attempts);
        self
    }
    pub fn audio_file(mut self, audio_file: impl Into<String>) -> Self {
        self.audio_file = Some(audio_file.into());
        self
    }
    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = Some(speed);
        self
    }
    pub fn build(self) -> AudioFactoryArgs {
        AudioFactoryArgs {
            name: self.name.unwrap_or(String::from("Unspecified Audio")),
            text: self.text.unwrap_or(String::from("Unspecified Text")),
            file_name: self.file_name.unwrap_or(String::from("unspecified.json")),
            audio_file: self.audio_file.unwrap_or(String::from("unspecified.mp3")),
            voice: self.voice.unwrap_or(TTSVoice::Nova),
            max_attempts: self.max_attempts.unwrap_or(3),
            speed: self.speed.unwrap_or(1.0),
        }
    }
}
