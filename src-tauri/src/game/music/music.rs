use serde::{Deserialize, Serialize};

use crate::game::selection_factory::Selectable;

use super::MusicMetadata;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Music {
    Selected {
        src: String,
        metadata: MusicMetadata,
    },
    #[serde(skip)]
    None,
}

impl Music {
    pub fn get_selection_input(meta_path: &str) -> Result<Vec<String>, anyhow::Error> {
        let meta_location = Music::get_meta_location(meta_path);
        let music_themes = MusicMetadata::load_from_file(meta_location)?;
        let music_themes = music_themes
            .into_iter()
            .map(|meta| format!("{}: {}", meta.index, meta.keywords))
            .collect();
        Ok(music_themes)
    }

    fn get_meta_location(meta_path: &str) -> String {
        // Because I know I'll end up forgetting the pattern
        match (meta_path.starts_with("/"), meta_path.ends_with("/")) {
            (true, true) => format!("../public{}meta.json", meta_path),
            (true, false) => format!("../public{}/meta.json", meta_path),
            (false, true) => format!("../public/{}meta.json", meta_path),
            (false, false) => format!("../public/{}/meta.json", meta_path),
        }
    }
}

impl Default for Music {
    fn default() -> Self {
        Music::None
    }
}

impl Selectable for Music {
    fn select_from_response(response: &String, meta_path: &str) -> Result<Self, anyhow::Error>
    where
        Self: Sized,
    {
        // Because I know I'll end up forgetting the pattern
        let meta_location = Music::get_meta_location(meta_path);
        // The find_by_index method uses pathbufs so the format doesn't matter
        let metadata = MusicMetadata::find_by_index(&meta_location, response.parse()?)?;
        Ok(Music::Selected {
            src: metadata.get_src(meta_path),
            metadata,
        })
    }
}
