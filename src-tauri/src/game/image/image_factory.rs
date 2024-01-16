use anyhow::anyhow;
use base64::{engine::general_purpose, Engine as _};
use log::info;
use openai_lib::{
    image::{CreateImageClient, CreateImageRequest, ImageObject, ImageQuality, ImageSize},
    model::image_model::ImageModel,
    OpenAIClient,
};

use crate::{file_manager::FileManager, game::game_metadata::GameMetadata, utils::random::Random};

use super::Image;

pub struct ImageFactory<'a> {
    openai_client: &'a OpenAIClient,
    file_manager: &'a FileManager,
    game_metadata: &'a GameMetadata,
    style: String,
}

impl<'a> ImageFactory<'a> {
    pub fn new(
        openai_client: &'a OpenAIClient,
        file_manager: &'a FileManager,
        game_metadata: &'a GameMetadata,
        style: String,
    ) -> Self {
        Self {
            openai_client,
            file_manager,
            game_metadata,
            style,
        }
    }

    pub async fn try_create(
        &self,
        image: &Image,
        factory_args: ImageFactoryArgs,
    ) -> Result<Image, anyhow::Error> {
        let mut errors = Vec::new();

        for _ in 0..factory_args.max_attempts {
            match self.create(&image, &factory_args).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    info!(
                        "Failed to create {}, trying again. Error: {:?}",
                        factory_args.filepath, &e
                    );
                    errors.push(e);
                }
            }
        }

        Err(anyhow!(
            "Failed to create {}. Max attempts exceeded. Attempts returned the following errors: {:?}.",
            factory_args.filepath,
            errors
        ))
    }

    async fn create(
        &self,
        image: &Image,
        factory_args: &ImageFactoryArgs,
    ) -> Result<Image, anyhow::Error> {
        match image {
            Image::Created { .. } => Ok(image.clone()),
            Image::Prompt(prompt) => self.generate_image(prompt, &factory_args).await,
        }
    }

    async fn generate_image(
        &self,
        prompt: &String,
        factory_args: &ImageFactoryArgs,
    ) -> Result<Image, anyhow::Error> {
        let modified_prompt = format!("{}\n{}", prompt, self.style);

        let data: Vec<ImageObject> = self
            .openai_client
            .create_image(
                CreateImageRequest::builder()
                    .prompt(&modified_prompt)
                    .model(factory_args.model.clone())
                    .quality(factory_args.quality.clone())
                    .size(factory_args.size.clone())
                    .b64_json()
                    .build(),
            )
            .await
            .map_err(|e| anyhow!("Failed to create image: {}", e))?
            .into();

        let alt = data[0]
            .revised_prompt
            .as_ref()
            .unwrap_or(&prompt.to_string())
            .clone();

        let base64_encoded = data[0]
            .b64_json
            .as_ref()
            .ok_or(anyhow!("B64 json not available."))?
            .split(",")
            .last()
            .ok_or(anyhow!("Failed to get base64 image."))?;

        let image_data = general_purpose::STANDARD
            .decode(base64_encoded)
            .map_err(|e| anyhow!("Failed to decode base64 image: {:?}", e))?;

        let filepath = format!("{}/{}", self.game_metadata.game_id, factory_args.filepath);

        let src = self
            .file_manager
            .write_bytes_to_file(&filepath, image_data)
            .map_err(|e| anyhow!("Failed to write image to file: {:?}", e))?;

        Ok(Image::Created { src, alt })
    }
}

#[derive(Debug, Clone)]
pub struct ImageFactoryArgs {
    pub model: ImageModel,
    pub quality: ImageQuality,
    pub size: ImageSize,
    pub filepath: String,
    pub max_attempts: usize,
}

impl ImageFactoryArgs {
    pub fn builder() -> ImageFactoryArgsBuilder {
        ImageFactoryArgsBuilder::new()
    }
}

pub struct ImageFactoryArgsBuilder {
    model: Option<ImageModel>,
    quality: Option<ImageQuality>,
    size: Option<ImageSize>,
    filepath: Option<String>,
    max_attempts: Option<usize>,
}

impl ImageFactoryArgsBuilder {
    pub fn new() -> Self {
        Self {
            model: None,
            quality: None,
            filepath: None,
            size: None,
            max_attempts: None,
        }
    }
    pub fn model(mut self, model: ImageModel) -> Self {
        self.model = Some(model);
        self
    }
    pub fn quality(mut self, quality: ImageQuality) -> Self {
        self.quality = Some(quality);
        self
    }
    pub fn filepath(mut self, filepath: impl Into<String>) -> Self {
        self.filepath = Some(filepath.into());
        self
    }
    pub fn size(mut self, size: ImageSize) -> Self {
        self.size = Some(size);
        self
    }
    pub fn max_attempts(mut self, max_attempts: usize) -> Self {
        self.max_attempts = Some(max_attempts);
        self
    }
    pub fn build(self) -> ImageFactoryArgs {
        ImageFactoryArgs {
            model: self.model.unwrap_or(ImageModel::DallE3),
            quality: self.quality.unwrap_or(ImageQuality::Standard),
            size: self.size.unwrap_or(ImageSize::Size1024x1024),
            max_attempts: self.max_attempts.unwrap_or(3),
            filepath: self
                .filepath
                .unwrap_or(format!("misc/{}", Random::generate_id())),
        }
    }
}
