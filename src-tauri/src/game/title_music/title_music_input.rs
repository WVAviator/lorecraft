use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::{
    audio::music_metadata::MusicMetadata,
    game::{summary::Summary, Image},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TitleMusicInput {
    game_summary: String,
    game_art_description: String,
    music_themes: Vec<String>,
}

impl TitleMusicInput {
    pub fn new(summary: &Summary) -> Result<Self, anyhow::Error> {
        let game_summary = summary.summary.clone();
        let game_art_description = match &summary.cover_art {
            Image::Prompt(prompt) => prompt.clone(),
            Image::Created { alt, .. } => alt.clone(),
        };

        let music_themes = MusicMetadata::load_from_file("../public/music/title/meta.json")?;
        let music_themes = music_themes
            .into_iter()
            .map(|meta| format!("{}: {}", meta.index, meta.keywords))
            .collect();

        Ok(TitleMusicInput {
            game_summary,
            game_art_description,
            music_themes,
        })
    }

    pub fn to_string(self) -> Result<String, anyhow::Error> {
        serde_json::to_string(&self)
            .map_err(|e| anyhow!("Unable to parse title music input to string: {:?}", e))
    }
}