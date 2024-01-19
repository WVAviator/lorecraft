use anyhow::anyhow;
use log::info;
use openai_lib::{
    chat_completion::{ChatCompletionClient, ChatCompletionRequest},
    model::ChatModel,
    OpenAIClient,
};
use serde::{Deserialize, Serialize};

use crate::{
    audio::music_metadata::MusicMetadata,
    file_manager::FileManager,
    game::{game_metadata::GameMetadata, summary::Summary},
    prompt_builder::PromptBuilder,
};

use super::title_music_input::TitleMusicInput;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TitleMusic {
    src: String,
    music_metadata: MusicMetadata,
}

impl TitleMusic {
    pub async fn try_create(
        summary: &Summary,
        openai_client: &OpenAIClient,
        game_metadata: &GameMetadata,
        file_manager: &FileManager,
    ) -> Result<Self, anyhow::Error> {
        let filename = format!("{}/tmp/title_music.json", &game_metadata.game_id);
        match file_manager.file_exists(&filename) {
            Ok(true) => {
                let title_music = file_manager.read_json::<TitleMusic>(filename)?;
                return Ok(title_music);
            }
            _ => {
                info!("No previous title music found, selecting new music.");
            }
        }

        let title_music = TitleMusic::create(summary, openai_client).await?;

        file_manager.write_json(filename, &title_music)?;

        Ok(title_music)
    }
    pub async fn create(
        summary: &Summary,
        openai_client: &OpenAIClient,
    ) -> Result<Self, anyhow::Error> {
        let system_message = PromptBuilder::new()
            .add_prompt("./prompts/title_music/main.txt")
            .build();
        let user_message = TitleMusicInput::new(summary)?;
        let user_message = user_message.to_string()?;

        for _ in 0..3 {
            match get_metadata(openai_client, &system_message, &user_message).await {
                Ok(music_metadata) => {
                    return Ok(TitleMusic {
                        src: music_metadata.get_src("/music/title/"),
                        music_metadata,
                    })
                }
                Err(err) => {
                    println!("Error getting metadata: {:?}", err);
                    continue;
                }
            }
        }

        Err(anyhow!("Failed to get metadata"))
    }
}

async fn get_metadata(
    openai_client: &OpenAIClient,
    system_message: &String,
    user_message: &String,
) -> Result<MusicMetadata, anyhow::Error> {
    let response = get_response(openai_client, system_message, user_message).await?;
    let metadata =
        MusicMetadata::find_by_index("../public/music/title/meta.json", response.parse()?)?;
    Ok(metadata)
}

async fn get_response(
    openai_client: &OpenAIClient,
    system_message: &String,
    user_message: &String,
) -> Result<String, anyhow::Error> {
    let response = openai_client
        .create_chat_completion(
            ChatCompletionRequest::builder()
                .add_system_message(system_message)
                .add_user_message(user_message)
                .model(ChatModel::Gpt_4_1106_Preview)
                .build(),
        )
        .await?;
    let content = response.get_content();
    Ok(content)
}
