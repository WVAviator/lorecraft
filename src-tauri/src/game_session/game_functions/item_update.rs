use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::game_state::GameState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemUpdate {
    pub success: bool,
    pub updated_player_inventory: Vec<String>,
}

impl ItemUpdate {
    pub fn add_item(
        arguments: serde_json::Value,
        game_state: &mut GameState,
    ) -> Result<Self, anyhow::Error> {
        let item = arguments["item"]
            .as_str()
            .ok_or(anyhow!("Invalid arguments to add_item function."))?
            .to_string();
        // TODO: Remove the item from the scene
        game_state.add_item(&item);
        let updated_player_inventory = game_state.get_player_inventory();

        Ok(ItemUpdate {
            success: true,
            updated_player_inventory,
        })
    }
    pub fn remove_item(
        arguments: serde_json::Value,
        game_state: &mut GameState,
    ) -> Result<Self, anyhow::Error> {
        let item = arguments["item"]
            .as_str()
            .ok_or(anyhow!("Invalid arguments to add_item function."))?
            .to_string();
        let mut success = false;
        if let Ok(_) = game_state.remove_item(&item) {
            success = true;
        }
        let updated_player_inventory = game_state.get_player_inventory();

        Ok(ItemUpdate {
            success,
            updated_player_inventory,
        })
    }
}
