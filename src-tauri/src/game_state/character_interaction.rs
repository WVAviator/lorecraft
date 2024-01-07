use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use super::{
    character_interaction_builder::CharacterInteractionBuilder,
    character_message::CharacterMessage, character_trade::CharacterTrade,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterInteraction {
    pub character_id: String,
    pub assistant_id: String,
    pub thread_id: String,
    pub initiating_run_id: String,
    pub initiating_tool_call_id: String,
    pub closed: bool,
    pub messages: Vec<CharacterMessage>,
    pub trade: Option<CharacterTrade>,
}

impl CharacterInteraction {
    pub fn builder() -> CharacterInteractionBuilder {
        CharacterInteractionBuilder::new()
    }

    pub fn add_message(&mut self, message: &str) {
        self.messages.push(CharacterMessage {
            text: message.to_string(),
            is_dialog: true,
        });
    }

    pub fn add_nonverbal(&mut self, message: &str) {
        self.messages.push(CharacterMessage {
            text: message.to_string(),
            is_dialog: false,
        });
    }

    pub fn propose_trade(&mut self, to_player: &str, from_player: &str) {
        self.trade = Some(CharacterTrade {
            to_player: Some(to_player.to_string()),
            from_player: Some(from_player.to_string()),
        });
    }

    pub fn propose_gift(&mut self, to_player: &str) {
        self.trade = Some(CharacterTrade {
            to_player: Some(to_player.to_string()),
            from_player: None,
        });
    }

    pub fn complete_trade(&mut self) {
        self.trade = None;
    }
}
