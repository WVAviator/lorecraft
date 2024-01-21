use serde::{Deserialize, Serialize};

use crate::game::music::Music;

use super::narrative::Page;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NarrativeMusicInput {
    narrative_pages: Vec<String>,
    music_themes: Vec<String>,
}

impl NarrativeMusicInput {
    pub fn new(pages: &Vec<Page>) -> Result<Self, anyhow::Error> {
        let narrative_pages = pages
            .iter()
            .enumerate()
            .map(|(index, page)| format!("Page {}: {}", index + 1, page.narrative))
            .collect();
        let music_themes = Music::get_selection_input("/music/narrative")?;

        Ok(Self {
            narrative_pages,
            music_themes,
        })
    }
}
