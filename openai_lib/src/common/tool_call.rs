use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    type_: String,
    function: FunctionOutput,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct FunctionOutput {
    name: String,
    arguments: String,
}

impl ToolCall {
    pub fn get_name(&self) -> String {
        self.function.name.clone()
    }
    pub fn extract_arguments<T: DeserializeOwned>(&self) -> Result<T, Error> {
        serde_json::from_str::<T>(&self.function.arguments)
            .map_err(|e| Error::DeserializationFailure(e))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn extracts_arguments_to_value() {
        let tool_call = ToolCall {
            id: String::from("test"),
            type_: String::from("test"),
            function: FunctionOutput {
                name: String::from("test"),
                arguments: String::from(r#"{"test": "test"}"#),
            },
        };
        let arguments = tool_call.extract_arguments::<serde_json::Value>().unwrap();
        assert_eq!(arguments["test"], "test");
    }

    #[test]
    fn extracts_arguments_to_struct() {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        struct Test {
            test: String,
        }

        let tool_call = ToolCall {
            id: String::from("test"),
            type_: String::from("test"),
            function: FunctionOutput {
                name: String::from("test"),
                arguments: String::from(r#"{"test": "test"}"#),
            },
        };
        let arguments = tool_call.extract_arguments::<Test>().unwrap();
        assert_eq!(arguments.test, "test");
    }
}
