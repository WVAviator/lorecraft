use serde::{Deserialize, Serialize};

use self::key_area::KeyArea;
use self::key_character::KeyCharacter;
use self::key_item::KeyItem;
use self::narrative::Narrative;
use self::player_attribute::PlayerAttribute;

pub mod key_area;
pub mod key_character;
pub mod key_item;
pub mod narrative;
pub mod player_attribute;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSummary {
    pub name: String,
    pub description: String,
    pub art_style: String,
    pub art_theme: String,
    pub cover_art: String,
    pub summary: String,
    pub win_condition: String,
    pub narrative: Vec<Narrative>,
    pub player_attributes: Vec<PlayerAttribute>,
    pub key_items: Vec<KeyItem>,
    pub key_areas: Vec<KeyArea>,
    pub key_characters: Vec<KeyCharacter>,
}

impl GameSummary {
    pub fn from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }
}
