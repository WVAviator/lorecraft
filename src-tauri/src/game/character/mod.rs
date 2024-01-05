use serde::{Deserialize, Serialize};

use self::character_output::CharacterOutput;

use super::image::Image;

pub mod character_factory;
mod character_input;
mod character_output;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    pub id: String,
    pub name: String,
    #[serde(default = "default_short_description")]
    pub short_description: String,
    pub physical_description: String,
    #[serde(default = "default_speech")]
    pub speech: String,
    pub personality: String,
    pub backstory: String,
    pub thoughts: String,
    pub inventory: Vec<String>,
    pub image: Image,
}

impl Character {
    pub fn new(character_output: CharacterOutput, id: String, image: Image) -> Self {
        Character {
            id,
            name: character_output.name,
            short_description: character_output.short_description,
            physical_description: character_output.physical_description,
            speech: character_output.speech,
            personality: character_output.personality,
            backstory: character_output.backstory,
            thoughts: character_output.thoughts,
            inventory: character_output.inventory,
            image,
        }
    }
}

fn default_short_description() -> String {
    String::from("a stranger")
}

fn default_speech() -> String {
    String::from("normal speaking voice")
}
