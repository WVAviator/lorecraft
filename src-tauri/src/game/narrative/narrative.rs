use futures::{StreamExt, TryStreamExt};
use log::info;
use openai_lib::{
    audio::TTSVoice,
    image::{ImageQuality, ImageSize},
    model::image_model::ImageModel,
};
use serde::{Deserialize, Serialize};

use crate::{
    config::content_setting::ContentSetting,
    file_manager::FileManager,
    game::{
        audio::{Audio, AudioFactory, AudioFactoryArgs},
        chat_completion_factory::{ChatCompletionFactory, ChatCompletionFactoryArgs},
        game_metadata::GameMetadata,
        image::{
            image_factory::{ImageFactory, ImageFactoryArgs},
            Image,
        },
        music::Music,
        selection_factory::{SelectionFactory, SelectionFactoryArgs},
        summary::Summary,
    },
    prompt_builder::PromptBuilder,
};

use super::narrative_music_input::NarrativeMusicInput;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Narrative {
    pages: Vec<Page>,
    #[serde(default, skip_serializing_if = "Music::is_none")]
    music: Music,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Page {
    pub narrative: String,
    pub image: Image,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,
}

impl Narrative {
    pub async fn create(
        summary: &Summary,
        chat_factory: &ChatCompletionFactory<'_>,
    ) -> Result<Self, anyhow::Error> {
        let system_prompt = PromptBuilder::new()
            .add_prompt("./prompts/narrative/main.txt")
            .add_example_input("./prompts/narrative/example1_input.json")
            .add_example_output("./prompts/narrative/example1_output.json")
            .add_example_input("./prompts/narrative/example2_input.json")
            .add_example_output("./prompts/narrative/example2_output.json")
            .build();

        let user_prompt = summary.summary.clone();

        info!("Prepared system and user messages for narrative.");

        let narrative = chat_factory
            .try_create::<Narrative>(
                ChatCompletionFactoryArgs::builder()
                    .name("Narrative")
                    .system_message(system_prompt)
                    .user_message(user_prompt)
                    .file_name("tmp/narrative.json")
                    .build(),
            )
            .await?;

        Ok(narrative)
    }

    pub async fn generate_audio(
        &mut self,
        audio_factory: &AudioFactory<'_>,
        game_metadata: &GameMetadata,
        file_manager: &FileManager,
    ) -> Result<(), anyhow::Error> {
        let mut futures = Vec::new();
        for (index, page) in self.pages.iter().enumerate() {
            let mut page = page.clone();
            let future = async move {
                let audio_file = format!("narrative/page_{}.mp3", index);
                let file_name = format!("tmp/narrative_audio/page_{}.json", index);
                let audio = audio_factory
                    .try_create(
                        AudioFactoryArgs::builder()
                            .name(format!("Narrative page {} audio", index))
                            .file_name(file_name)
                            .audio_file(audio_file)
                            .voice(TTSVoice::Nova)
                            .text(page.narrative.clone())
                            .speed(0.8f32)
                            .build(),
                    )
                    .await?;

                page.audio = Some(audio.clone());

                file_manager
                    .json_transaction::<Narrative, _>(
                        format!("{}/tmp/narrative.json", game_metadata.game_id),
                        move |mut narrative| {
                            if let Some(page) = narrative.pages.get_mut(index) {
                                page.audio = Some(audio.clone());
                            }
                            narrative
                        },
                    )
                    .await?;

                Ok(page) as Result<Page, anyhow::Error>
            };

            futures.push(future);
        }

        let stream = futures::stream::iter(futures).buffered(10);
        let pages = stream.try_collect::<Vec<_>>().await?;

        self.pages = pages;

        Ok(())
    }

    pub async fn generate_images(
        &mut self,
        factory: &ImageFactory<'_>,
        game_metadata: &GameMetadata,
        file_manager: &FileManager,
    ) -> Result<(), anyhow::Error> {
        let mut futures = Vec::new();

        info!("Generating images for narrative pages.");

        for (index, page) in self.pages.iter().enumerate() {
            let page = page.clone();
            let future = async move {
                info!("Generating image for narrative page: {}", index);

                if let Image::Created { .. } = page.image {
                    info!("Image already created and saved - skipping.");
                    return Ok(page);
                }

                let (model, quality) = match game_metadata.image_content_setting {
                    ContentSetting::High => (ImageModel::DallE3, ImageQuality::HD),
                    _ => (ImageModel::DallE3, ImageQuality::Standard),
                };

                let filepath = format!("narrative/page_{}.png", index);

                let image = factory
                    .try_create(
                        &page.image,
                        ImageFactoryArgs::builder()
                            .size(ImageSize::Size1792x1024)
                            .model(model)
                            .quality(quality)
                            .filepath(filepath)
                            .build(),
                    )
                    .await?;

                info!("Image generated for narrative page {}. Saving new iamge data to narrative JSON file.", index);

                file_manager
                    .json_transaction::<Narrative, _>(
                        format!("{}/tmp/narrative.json", game_metadata.game_id),
                        move |mut narrative| {
                            if let Some(page) = narrative.pages.get_mut(index) {
                                page.image = image.clone();
                            }
                            narrative
                        },
                    )
                    .await?;

                Ok(page) as Result<Page, anyhow::Error>
            };
            futures.push(future);
        }

        let stream = futures::stream::iter(futures).buffered(3);
        let pages = stream.try_collect::<Vec<_>>().await?;

        info!("All narrative page images generated and saved.");

        self.pages = pages;

        Ok(())
    }

    pub async fn select_music(
        &mut self,
        factory: &SelectionFactory<'_>,
        game_metadata: &GameMetadata,
        file_manager: &FileManager,
    ) -> Result<(), anyhow::Error> {
        let meta_path = String::from("music/narrative/");

        let system_message = PromptBuilder::new()
            .add_prompt("./prompts/narrative_music/main.txt")
            .build();

        let user_message = NarrativeMusicInput::new(&self.pages)?;
        let user_message = serde_json::to_string(&user_message)?;

        info!("Selecting narrative music.");

        let music = factory
            .try_create::<Music>(
                SelectionFactoryArgs::builder()
                    .name("Narrative music")
                    .system_message(system_message)
                    .user_message(user_message)
                    .file_name("narrative/music.json")
                    .meta_path(meta_path)
                    .build(),
            )
            .await?;

        self.music = music.clone();

        file_manager
            .json_transaction::<Narrative, _>(
                format!("{}/tmp/narrative.json", game_metadata.game_id),
                move |mut narrative| {
                    narrative.music = music;
                    narrative
                },
            )
            .await?;

        Ok(())
    }
}
