use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};

use crate::{file_manager::FileManager, game::Game};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSaveData {
    character_id: String,
    game_id: String,
    pub previous_conversations: Vec<String>,
    pub character_inventory: Vec<String>,
}

impl CharacterSaveData {
    pub fn load(
        character_id: &str,
        game_id: &str,
        file_manager: &FileManager,
    ) -> Result<Self, anyhow::Error> {
        let path = format!("save_data/{}/character_data/{}.json", game_id, character_id);
        match file_manager.file_exists(&path) {
            Ok(true) => {
                let json = file_manager.read_from_file(&path)?;
                serde_json::from_str::<Self>(&json).context("Invalid character data.")
            }
            Ok(false) => {
                let game_path = format!("{}/game.json", game_id);
                let game_json = file_manager.read_from_file(&game_path)?;
                let game = serde_json::from_str::<Game>(&game_json)?;
                let character_inventory = game
                    .characters
                    .iter()
                    .find(|c| c.id == character_id)
                    .context("Character not found in game.")?
                    .inventory
                    .clone();

                let save_data = CharacterSaveData {
                    character_id: character_id.to_string(),
                    game_id: game_id.to_string(),
                    character_inventory,
                    previous_conversations: vec![],
                };

                save_data.save(file_manager)?;

                Ok(save_data)
            }
            Err(_) => Err(anyhow!("Unable to verify character save data.")),
        }
    }

    pub fn save(&self, file_manager: &FileManager) -> Result<(), anyhow::Error> {
        let path = format!(
            "save_data/{}/character_data/{}.json",
            self.game_id, self.character_id
        );
        let json = serde_json::to_string(self)?;
        file_manager.write_to_file(&path, &json)?;

        Ok(())
    }
}
