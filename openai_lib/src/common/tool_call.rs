use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    type_: String,
    function: FunctionOutput,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FunctionOutput {
    name: String,
    arguments: String,
}

impl ToolCall {
    pub fn get_name(&self) -> String {
        self.function.name.clone()
    }
    pub fn extract_arguments<T: DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_str::<T>(&self.function.arguments)
    }
}
