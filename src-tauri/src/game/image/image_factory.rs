use std::cell::RefCell;

use anyhow::{anyhow, bail};
use base64::{engine::general_purpose, Engine as _};
use log::{error, info, trace};
use openai_lib::{
    image::{CreateImageClient, CreateImageRequest, ImageObject, ResponseFormat},
    OpenAIClient,
};

use crate::{file_manager::FileManager, game::game_metadata::GameMetadata};

use super::Image;

pub struct ImageFactory<'a> {
    openai_client: &'a OpenAIClient,
    file_manager: &'a FileManager,
    game_metadata: &'a GameMetadata,
    style: RefCell<Option<String>>,
}

impl<'a> ImageFactory<'a> {
    pub fn new(
        openai_client: &'a OpenAIClient,
        file_manager: &'a FileManager,
        game_metadata: &'a GameMetadata,
    ) -> Self {
        Self {
            openai_client,
            file_manager,
            game_metadata,
            style: RefCell::new(None),
        }
    }

    pub fn add_style(&self, style: impl Into<String>) {
        *self.style.borrow_mut() = Some(style.into());
    }

    // pub async fn try_create(&self, factory_args: ImageFactoryArgs) -> Result<Image, anyhow::Error> {
    //     info!("Creating image '{}'.", factory_args.name);

    //     let mut errors = Vec::new();

    //     for _ in 0..factory_args.max_attempts {
    //         match self.create(&factory_args).await {
    //             Ok(image) => return Ok(image),
    //             Err(e) => {
    //                 info!(
    //                     "Failed to create image '{}', trying again. Error: {:?}",
    //                     factory_args.name, &e
    //                 );
    //                 errors.push(e);
    //             }
    //         }
    //     }

    //     Err(anyhow!(
    //         "Failed to create image '{}'. Max attempts exceeded. Attempts returned the following errors: {:?}.",
    //         factory_args.name,
    //         errors
    //     ))
    // }

    // async fn create(&self, factory_args: &ImageFactoryArgs) -> Result<Image, anyhow::Error> {}

    pub async fn try_generate_image(
        &self,
        create_image_request: CreateImageRequest,
        filepath: &str,
        max_attempts: u32,
    ) -> Result<Image, anyhow::Error> {
        let mut attempts = 0;
        while attempts < max_attempts {
            match self
                .generate_image(create_image_request.clone(), filepath)
                .await
            {
                Ok(image) => return Ok(image),
                Err(e) => {
                    error!("Image API request failed with reason: {:?}", e);
                    trace!("Request:\n{:?}", &create_image_request);
                    info!("Retrying...")
                }
            }
            attempts += 1;
        }

        bail!("Max attempts exceeded.");
    }

    async fn generate_image(
        &self,
        mut create_image_request: CreateImageRequest,
        filepath: &str,
    ) -> Result<Image, anyhow::Error> {
        let style = self
            .style
            .borrow()
            .as_ref()
            .unwrap_or(&String::from("In the style of digital art"))
            .clone();

        create_image_request.modify_prompt(|prompt| format!("{}\n{}", prompt, style));
        create_image_request.modify_response_format(ResponseFormat::B64Json);
        let prompt = create_image_request.get_prompt();

        let data: Vec<ImageObject> = self
            .openai_client
            .create_image(create_image_request)
            .await?
            .into();

        let alt = data[0].revised_prompt.as_ref().unwrap_or(&prompt).clone();

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

        let filepath = format!("{}/{}", self.game_metadata.game_id, filepath);

        let src = self
            .file_manager
            .write_bytes_to_file(&filepath, image_data)
            .map_err(|e| anyhow!("Failed to write image to file: {:?}", e))?;

        Ok(Image::Created { src, alt })
    }
}
