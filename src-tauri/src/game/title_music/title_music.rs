use serde::{Deserialize, Serialize};

use crate::{
    audio::music_metadata::MusicMetadata,
    game::{
        selection_factory::{Selectable, SelectionFactory, SelectionFactoryArgs},
        summary::Summary,
    },
    prompt_builder::PromptBuilder,
};

use super::title_music_input::TitleMusicInput;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TitleMusic {
    src: String,
    music_metadata: MusicMetadata,
}

impl Selectable for TitleMusic {
    fn select_from_response(response: &String) -> Result<Self, anyhow::Error>
    where
        Self: Sized,
    {
        let music_metadata =
            MusicMetadata::find_by_index("../public/music/title/meta.json", response.parse()?)?;
        Ok(TitleMusic {
            src: music_metadata.get_src("/music/title/"),
            music_metadata,
        })
    }
}

impl TitleMusic {
    pub async fn create(
        summary: &Summary,
        factory: &SelectionFactory<'_>,
    ) -> Result<Self, anyhow::Error> {
        let system_message = PromptBuilder::new()
            .add_prompt("./prompts/title_music/main.txt")
            .build();
        let user_message = TitleMusicInput::new(summary)?;
        let user_message = user_message.to_string()?;

        let title_music = factory
            .try_create(
                SelectionFactoryArgs::builder()
                    .system_message(system_message)
                    .user_message(user_message)
                    .name("Title Music")
                    .file_name("tmp/title_music.json")
                    .build(),
            )
            .await?;

        Ok(title_music)
    }
}
