use serde::{Deserialize, Serialize};

use crate::file_manager::FileManager;

use super::{summary::Summary, title_music::TitleMusic, Character, Image, Item, Narrative, Scene};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub name: String,
    pub summary: Summary,
    pub cover_art: Image,
    pub narrative: Narrative,
    pub scenes: Vec<Scene>,
    pub characters: Vec<Character>,
    pub items: Vec<Item>,
    pub title_music: TitleMusic,
}

impl Game {
    pub fn load(
        game_id: impl Into<String>,
        file_manager: &FileManager,
    ) -> Result<Self, anyhow::Error> {
        file_manager.read_json::<Game>(format!("{}/game.json", game_id.into()))
    }
}
