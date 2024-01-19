use serde::{Deserialize, Serialize};

use super::music_metadata::MusicMetadata;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Music {
    src: String,
    metadtata: MusicMetadata,
}
