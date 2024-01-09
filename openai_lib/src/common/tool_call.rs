use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCall {
    id: String,
    #[serde(rename = "type")]
    type_: String,
    function: Function,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Function {
    name: String,
    arguments: String,
}

impl ToolCall {
    pub fn extract_arguments<T = serde_json::Value>(&self) -> Result<T, serde::de::Error> 
        where T: Deserialize {
        serde_json::from_str::<T>(&self.function.arguments)
    }
}
