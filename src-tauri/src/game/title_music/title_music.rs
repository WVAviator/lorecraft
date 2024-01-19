use serde::{Deserialize, Serialize};

use crate::audio::music_metadata::MusicMetadata;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TitleMusic {
    src: String,
    music_metadta: MusicMetadata,
}
