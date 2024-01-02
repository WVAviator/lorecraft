use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub parameters: Parameters,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct Parameters {
    #[serde(rename = "type")]
    pub type_: String,
    pub properties: HashMap<String, Property>,
    pub required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
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
