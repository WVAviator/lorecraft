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
    pub physical_description: String,
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
            physical_description: character_output.physical_description,
            personality: character_output.personality,
            backstory: character_output.backstory,
            thoughts: character_output.thoughts,
            inventory: character_output.inventory,
            image,
        }
    }
}
