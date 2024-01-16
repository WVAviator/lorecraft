use anyhow::{anyhow, Context};
use futures::{StreamExt, TryStreamExt};
use log::info;
use openai_lib::{
    chat_completion::{ChatCompletionClient, ChatCompletionRequest},
    image::{CreateImageRequest, ImageQuality, ImageSize},
    model::{image_model::ImageModel, ChatModel},
    OpenAIClient,
};

use crate::{
    commands::create_new_game::create_new_game_request::CreateNewGameRequest,
    config::content_setting::ContentSetting,
    file_manager::FileManager,
    game::{game_metadata::GameMetadata, image::image_factory::ImageFactory, summary::Summary},
    prompt_builder::PromptBuilder,
};

use super::{
    narrative_output::{NarrativeOutput, OutputPage},
    Narrative, Page,
};

pub struct NarrativeFactory<'a> {
    openai_client: &'a OpenAIClient,
    file_manager: &'a FileManager,
    image_factory: &'a ImageFactory<'a>,
    game_metadata: &'a GameMetadata,
}

impl<'a> NarrativeFactory<'a> {
    pub fn new(
        openai_client: &'a OpenAIClient,
        file_manager: &'a FileManager,
        image_factory: &'a ImageFactory<'a>,
        game_metadata: &'a GameMetadata,
    ) -> Self {
        Self {
            openai_client,
            file_manager,
            image_factory,
            game_metadata,
        }
    }

    pub async fn try_create(&self, summary: &Summary) -> Result<Narrative, anyhow::Error> {
        info!("Creating game narrative.");
        let mut errors = Vec::new();
        for _ in 0..3 {
            match self.create(summary).await {
                Ok(narrative) => return Ok(narrative),
                Err(e) => {
                    info!("Failed to create narrative, trying again. Error: {:?}", &e);
                    errors.push(e);
                }
            }
        }
        Err(anyhow!(
            "Failed to create narrative. Max attempts exceeded. Attempts returned the following errors: {:?}.",
            errors
        ))
    }

    async fn create(&self, summary: &Summary) -> Result<Narrative, anyhow::Error> {
        let narrative_path = format!("{}/tmp/narrative.json", &self.game_metadata.game_id);

        info!(
            "Checking for existing narrative JSON file at {}",
            &narrative_path
        );

        match self.file_manager.file_exists(&narrative_path) {
            Ok(true) => {
                return self
                    .file_manager
                    .read_json::<Narrative>(&narrative_path)
                    .context("Unable to read existing narrative JSON file.");
            }
            _ => {
                info!("No existing narrative found, generating new narrative.");
            }
        }

        let narrative_output = self.create_narrative_output(summary).await?;
        let narrative = self.generate_page_images(narrative_output).await?;

        self.file_manager
            .write_json::<Narrative>(&narrative_path, &narrative)
            .context("Unable to write narrative JSON file.")?;

        info!("Finished generating narrative pages.");
        Ok(narrative)
    }

    async fn create_narrative_output(
        &self,
        summary: &Summary,
    ) -> Result<NarrativeOutput, anyhow::Error> {
        let narrative_output_path =
            format!("{}/tmp/narrative_output.json", &self.game_metadata.game_id);

        info!(
            "Checking for existing narrative output JSON file at {}",
            &narrative_output_path
        );

        match self.file_manager.file_exists(&narrative_output_path) {
            Ok(true) => {
                return self
                    .file_manager
                    .read_json::<NarrativeOutput>(&narrative_output_path)
                    .context("Unable to read existing narrative output JSON file.");
            }
            _ => {
                info!("No existing narrative output found, generating new narrative output.");
            }
        }

        let narrative_output = self.generate_narrative_output(summary).await?;

        self.file_manager
            .write_json::<NarrativeOutput>(&narrative_output_path, &narrative_output)
            .context("Unable to write summary JSON file.")?;

        info!("Generated narrative text and image descriptions.");

        Ok(narrative_output)
    }

    async fn generate_narrative_output(
        &self,
        summary: &Summary,
    ) -> Result<NarrativeOutput, anyhow::Error> {
        let system_prompt = PromptBuilder::new()
            .add_prompt("./prompts/narrative/main.txt")
            .add_example_input("./prompts/narrative/example1_input.json")
            .add_example_output("./prompts/narrative/example1_output.json")
            .add_example_input("./prompts/narrative/example2_input.json")
            .add_example_output("./prompts/narrative/example2_output.json")
            .build();

        let user_prompt = summary.summary.clone();

        let model = match self.game_metadata.text_content_setting {
            ContentSetting::Low => ChatModel::Gpt_35_Turbo_1106,
            _ => ChatModel::Gpt_4_1106_Preview,
        };

        info!("Generating opening narrative for game.");

        let response_text = self
            .openai_client
            .create_chat_completion(
                ChatCompletionRequest::builder()
                    .add_system_message(system_prompt)
                    .add_user_message(user_prompt)
                    .model(model)
                    .temperature(self.game_metadata.temperature_setting)
                    .json()
                    .build(),
            )
            .await
            .map_err(|e| anyhow!("Failed to create chat completion: {}", e))?
            .get_content();

        let narrative_output = serde_json::from_str::<NarrativeOutput>(response_text.as_str())
            .map_err(|e| anyhow!("Failed to deserialize narrative: {}", e))?;

        Ok(narrative_output)
    }

    async fn generate_page_images(
        &self,
        narrative_output: NarrativeOutput,
    ) -> Result<Narrative, anyhow::Error> {
        let mut page_futures = Vec::new();

        for (index, page) in narrative_output.pages.iter().enumerate() {
            let page_future = self.create_image(index, page);
            page_futures.push(page_future);
        }

        let stream = futures::stream::iter(page_futures).buffered(3);
        let pages = stream.try_collect::<Vec<_>>().await?;

        Ok(Narrative::new(pages))
    }

    async fn create_image(&self, index: usize, page: &OutputPage) -> Result<Page, anyhow::Error> {
        let narrative_output_path =
            format!("{}/tmp/narrative_output.json", &self.game_metadata.game_id);

        let narrative_output = self
            .file_manager
            .read_json::<NarrativeOutput>(&narrative_output_path)
            .context("Unable to read existing narrative output JSON file.")?;

        if let Some(image) = &narrative_output
            .pages
            .get(index)
            .ok_or(anyhow!("Could not find page at index {}.", index))?
            .image_object
        {
            info!("Using existing image for narrative page {}.", index);
            return Ok(Page::new(page.narrative.clone(), image.clone()));
        }

        let page = self.generate_page(index, &page).await?;
        let image = page.image.clone();

        self.file_manager.json_transaction::<NarrativeOutput, _>(
            &narrative_output_path,
            move |mut narrative_output| {
                if let Some(output_page) = narrative_output.pages.get_mut(index) {
                    output_page.image_object = Some(image);
                }
                narrative_output
            },
        );

        Ok(page)
    }

    async fn generate_page(&self, index: usize, page: &OutputPage) -> Result<Page, anyhow::Error> {
        let filepath = format!("narrative/page_{}.png", index);

        let quality = match self.game_metadata.image_content_setting {
            ContentSetting::High => ImageQuality::HD,
            _ => ImageQuality::Standard,
        };

        info!("Generating image for narrative page {}.", index);

        let image = self
            .image_factory
            .try_generate_image(
                CreateImageRequest::builder()
                    .prompt(&page.image)
                    .model(ImageModel::DallE3)
                    .size(ImageSize::Size1792x1024)
                    .quality(quality)
                    .build(),
                &filepath,
                3,
            )
            .await
            .map_err(|e| anyhow!("Failed to generate narrative image: {}", e))?;

        info!(
            "Finished generating narrative page and image for index {}.",
            index
        );

        Ok(Page::new(page.narrative.clone(), image))
    }
}
