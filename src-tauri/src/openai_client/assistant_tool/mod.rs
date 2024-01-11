pub mod function;

use serde::{Deserialize, Serialize};

use self::function::Function;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssistantTool {
    #[serde(rename = "type")]
    pub type_: String,
    pub function: Option<Function>,
}

impl AssistantTool {
    pub fn new_function(function: Function) -> Self {
        AssistantTool {
            type_: String::from("function"),
            function: Some(function),
        }
    }
}
