use log::warn;
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
    #[serde(default = "default_name")]
    pub name: String,
    #[serde(default = "default_description")]
    pub description: String,
    #[serde(default = "default_art_style")]
    pub art_style: String,
    #[serde(default = "default_art_theme")]
    pub art_theme: String,
    #[serde(default = "default_cover_art")]
    pub cover_art: String,
    #[serde(default = "default_summary")]
    pub summary: String,
    #[serde(default = "default_win_condition")]
    pub win_condition: String,
    #[serde(default)]
    pub narrative: Vec<Narrative>,
    #[serde(default)]
    pub player_attributes: Vec<PlayerAttribute>,
    #[serde(default)]
    pub key_items: Vec<KeyItem>,
    #[serde(default)]
    pub key_areas: Vec<KeyArea>,
    #[serde(default)]
    pub key_characters: Vec<KeyCharacter>,
}

impl GameSummary {
    pub fn from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }
}

fn default_name() -> String {
    warn!("AI-provided game summary name is missing.");
    String::new()
}

fn default_description() -> String {
    warn!("AI-provided game summary description is missing.");
    String::new()
}

fn default_art_style() -> String {
    warn!("AI-provided game summary art style is missing.");
    String::new()
}

fn default_art_theme() -> String {
    warn!("AI-provided game summary art theme is missing.");
    String::new()
}

fn default_cover_art() -> String {
    warn!("AI-provided game summary cover art is missing.");
    String::new()
}

fn default_summary() -> String {
    warn!("AI-provided game summary summary is missing.");
    String::new()
}

fn default_win_condition() -> String {
    warn!("AI-provided game summary win condition is missing.");
    String::new()
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    fn get_test_yaml(file: &str) -> String {
        fs::read_to_string(file).unwrap()
    }

    #[test]
    fn game_summary_parses_yaml_successfully() {
        let yaml = get_test_yaml("./prompts/testing/game_summary.yaml");
        let game_summary = GameSummary::from_yaml(&yaml).unwrap();
        assert_eq!(game_summary.name, "Test Game Summary");
        assert_eq!(game_summary.description, "A test.");
        assert_eq!(game_summary.art_style, "Test art style.");
        assert_eq!(game_summary.art_theme, "Test art theme.");
        assert_eq!(game_summary.cover_art, "Test cover art.");
        assert_eq!(game_summary.summary, "Test summary.");
        assert_eq!(game_summary.win_condition, "Test win condition.");
        assert_eq!(game_summary.narrative.len(), 2);
        assert_eq!(game_summary.player_attributes.len(), 2);
        assert_eq!(game_summary.key_items.len(), 1);
        assert_eq!(game_summary.key_areas.len(), 2);
        assert_eq!(game_summary.key_characters.len(), 1);

        assert_eq!(game_summary.narrative[0].narrative, "Test narrative.");
        assert_eq!(game_summary.narrative[0].image, "Test image.");
        assert_eq!(game_summary.narrative[1].narrative, "Test narrative.");
        assert_eq!(game_summary.narrative[1].image, "Test image.");

        assert_eq!(game_summary.player_attributes[0].name, "Test Attribute");
        assert_eq!(
            game_summary.player_attributes[0].description,
            "Test attribute description."
        );
        assert_eq!(
            game_summary.player_attributes[0].advancement,
            "Test attribute advancement."
        );
        assert_eq!(
            game_summary.player_attributes[0].significance,
            "Test attribute significance."
        );
        assert_eq!(game_summary.player_attributes[0].starting_value, 1);

        assert_eq!(game_summary.player_attributes[1].name, "Test Attribute 2");
        assert_eq!(
            game_summary.player_attributes[1].description,
            "Test attribute 2 description."
        );
        assert_eq!(
            game_summary.player_attributes[1].advancement,
            "Test attribute 2 advancement."
        );
        assert_eq!(
            game_summary.player_attributes[1].significance,
            "Test attribute 2 significance."
        );
        assert_eq!(game_summary.player_attributes[1].starting_value, 4);

        assert_eq!(game_summary.key_items[0].name, "Test Key Item");
        assert_eq!(
            game_summary.key_items[0].description,
            "Test key item description."
        );
        assert_eq!(
            game_summary.key_items[0].location,
            "Test key item location."
        );
        assert_eq!(
            game_summary.key_items[0].significance,
            "Test key item significance."
        );
        assert_eq!(game_summary.key_items[0].image, "Test key item image.");

        assert_eq!(game_summary.key_areas[0].name, "Test Key Area");
        assert_eq!(
            game_summary.key_areas[0].description,
            "Test key area description."
        );
        assert_eq!(
            game_summary.key_areas[0].significance,
            "Test key area significance."
        );

        assert_eq!(game_summary.key_areas[1].name, "Test Key Area 2");
        assert_eq!(
            game_summary.key_areas[1].description,
            "Test key area 2 description."
        );
        assert_eq!(
            game_summary.key_areas[1].significance,
            "Test key area 2 significance."
        );

        assert_eq!(game_summary.key_characters[0].name, "Test Key Character");
        assert_eq!(
            game_summary.key_characters[0].description,
            "Test key character description."
        );
        assert_eq!(
            game_summary.key_characters[0].significance,
            "Test key character significance."
        );
        assert_eq!(
            game_summary.key_characters[0].image,
            "Test key character image."
        );
    }

    #[test]
    fn game_summary_parses_incomplete_yaml() {
        let yaml = get_test_yaml("./prompts/testing/game_summary_incomplete.yaml");
        let game_summary = GameSummary::from_yaml(&yaml).unwrap();

        assert_eq!(game_summary.name, "Test Game Summary");
        assert_eq!(game_summary.description, "");
        assert_eq!(game_summary.player_attributes[0].description, "");
        assert_eq!(game_summary.key_areas.len(), 0);
        assert_eq!(game_summary.key_items.len(), 0);
    }

    #[test]
    fn game_summary_prompt_examples_valid() {
        let example1_yaml = get_test_yaml("./prompts/architect/example1.yaml");
        let example2_yaml = get_test_yaml("./prompts/architect/example2.yaml");

        let example1_game_summary = GameSummary::from_yaml(&example1_yaml).unwrap();
        let example2_game_summary = GameSummary::from_yaml(&example2_yaml).unwrap();
    }
}
