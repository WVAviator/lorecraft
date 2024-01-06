pub mod character_interaction;
pub mod character_message;
pub mod character_trade;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::game::Game;

use self::character_interaction::CharacterInteraction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameState {
    pub game_id: String,
    pub current_scene_id: Option<String>,
    pub messages: Vec<String>,
    pub inventory: Vec<String>,
    pub character_interaction: Option<CharacterInteraction>,
    pub character_inventories: HashMap<String, Vec<String>>,
    pub scene_inventories: HashMap<String, Vec<String>>,
    pub assistant_id: String,
    pub thread_id: String,
    pub end_game: Option<String>,
}

impl GameState {
    pub fn new(game: &Game, assistant_id: &str, thread_id: &str) -> Self {
        let game_id = game.id.clone();

        let character_inventories = game
            .characters
            .iter()
            .map(|c| (c.id.clone(), c.inventory.clone()))
            .collect::<HashMap<String, Vec<String>>>();

        let scene_inventories = game
            .scenes
            .iter()
            .map(|s| (s.id.clone(), s.items.clone()))
            .collect::<HashMap<String, Vec<String>>>();

        GameState {
            game_id,
            current_scene_id: None,
            messages: vec![],
            inventory: vec![],
            character_interaction: None,
            character_inventories,
            scene_inventories,
            assistant_id: assistant_id.to_string(),
            thread_id: thread_id.to_string(),
            end_game: None,
        }
    }

    pub fn add_player_message(&mut self, message: &str) {
        self.messages.push(format!("> {}", message));
    }

    pub fn add_narrator_message(&mut self, message: &str) {
        self.messages.push(message.to_string());
    }

    pub fn new_scene(&mut self, new_scene_id: &str) {
        self.current_scene_id = Some(new_scene_id.to_string());
    }

    pub fn add_item(&mut self, item_name: &str) {
        self.inventory.push(item_name.to_string());
    }

    pub fn remove_item(&mut self, item_id: &str) -> Result<(), anyhow::Error> {
        let index = self
            .inventory
            .iter()
            .position(|i| i == item_id)
            .ok_or_else(|| anyhow::anyhow!("Item '{}' not found in inventory.", item_id))?;

        self.inventory.remove(index);

        Ok(())
    }

    pub fn character_interact(&mut self, character_id: &str) {
        self.character_interaction = Some(CharacterInteraction::new(character_id));
    }

    pub fn end_character_interaction(&mut self) {
        self.character_interaction = None;
    }

    pub fn get_inventory(&self) -> Vec<String> {
        self.inventory.clone()
    }
}
