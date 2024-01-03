use std::collections::HashMap;

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::prompt_builder::PromptBuilder;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Parameters,
    pub description: String,
}

impl Function {
    pub fn to_json(&self) -> Result<String, anyhow::Error> {
        serde_json::to_string(&self).context("Unable to serialize function.")
    }

    pub fn from_file(file_path: &str) -> Result<Self, anyhow::Error> {
        let json = PromptBuilder::new().add_prompt(file_path).build();

        serde_json::from_str::<Self>(&json).context("Unable to deserialize function.")
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Parameters {
    #[serde(rename = "type")]
    pub type_: String,
    pub properties: HashMap<String, Property>,
    pub required: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Property {
    #[serde(rename = "type")]
    pub type_: String,
    pub description: String,
}

// {
//   "name": "give_item",
//   "parameters": {
//     "type": "object",
//     "properties": {
//       "item": {
//         "type": "string",
//         "description": "The item you wish to give. It should exist in your inventory."
//       }
//     },
//     "required": ["item"]
//   },
//   "description": "Give an item from your inventory to the player."
// }
