use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tool {
    #[serde(rename = "type")]
    type_: String,
    function: Option<Function>,
}

impl Tool {
    pub fn code_interpreter() -> Self {
        Tool {
            type_: String::from("code_interpreter"),
            function: None,
        }
    }

    pub fn retrieval() -> Self {
        Tool {
            type_: String::from("retrieval"),
            function: None,
        }
    }

    pub fn function() -> FunctionBuilder {
        FunctionBuilder::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Function {
    description: Option<String>,
    name: String,
    parameters: Option<FunctionParameters>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FunctionParameters {
    #[serde(rename = "type")]
    type_: String,
    properties: HashMap<String, FunctionProperty>,
    required: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FunctionProperty {
    #[serde(rename = "type")]
    type_: String,
    description: String,
}

pub struct FunctionBuilder {
    name: Option<String>,
    description: Option<String>,
    parameters: Option<FunctionParameters>,
}

impl FunctionBuilder {
    fn new() -> Self {
        FunctionBuilder {
            name: None,
            description: None,
            parameters: None,
        }
    }

    pub fn from_json(self, json: &str) -> Result<Tool, Error> {
        let function =
            serde_json::from_str::<Function>(json).map_err(|e| Error::DeserializationFailure(e))?;

        Ok(Tool {
            type_: String::from("function"),
            function: Some(function),
        })
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn add_property(
        mut self,
        name: &str,
        description: &str,
        property_type: &str,
        required: bool,
    ) -> Self {
        if let None = self.parameters {
            self.parameters = Some(FunctionParameters {
                type_: String::from("object"),
                properties: HashMap::new(),
                required: Vec::new(),
            });
        }

        let parameters = self.parameters.as_mut().unwrap();

        parameters.properties.insert(
            name.to_string(),
            FunctionProperty {
                type_: property_type.to_string(),
                description: description.to_string(),
            },
        );

        if required {
            parameters.required.push(name.to_string());
        }

        self
    }

    pub fn build(self) -> Result<Tool, Error> {
        let name = self
            .name
            .ok_or(Error::MissingRequiredProperty(String::from("name")))?;

        Ok(Tool {
            type_: String::from("function"),
            function: Some(Function {
                name,
                description: self.description,
                parameters: self.parameters,
            }),
        })
    }
}

#[cfg(test)]
mod test {
    use assert_json_diff::assert_json_include;
    use serde_json::json;

    use super::*;

    #[test]
    fn returns_code_interpreter() {
        let tool = Tool::code_interpreter();
        assert_eq!(tool.type_, String::from("code_interpreter"));
        assert!(tool.function.is_none());
    }

    #[test]
    fn returns_retrieval() {
        let tool = Tool::retrieval();
        assert_eq!(tool.type_, String::from("retrieval"));
        assert!(tool.function.is_none());
    }

    #[test]
    fn function_builder_returns_function() {
        let tool = Tool::function()
            .name("test")
            .description("test")
            .add_property("test", "test", "string", true)
            .build()
            .unwrap();

        let expected_json = json!({
            "type": "function",
            "function": {
                "name": "test",
                "description": "test",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "test": {
                            "type": "string",
                            "description": "test"
                        }
                    },
                    "required": ["test"]
                }
            }
        });

        let tool_json = serde_json::to_value(&tool).unwrap();

        assert_json_include!(actual: tool_json, expected: expected_json);
    }

    #[test]
    fn builds_function_from_json_data() {
        let function_json_string = json!({
            "name": "test",
            "description": "test",
            "parameters": {
                "type": "object",
                "properties": {
                    "test": {
                        "type": "string",
                        "description": "test"
                    }
                },
                "required": ["test"]
            }
        })
        .to_string();

        let tool = Tool::function().from_json(&function_json_string).unwrap();

        assert_eq!(tool.type_, String::from("function"));
        assert!(tool.function.is_some());
        assert_eq!(tool.function.as_ref().unwrap().name, String::from("test"));
        assert_eq!(
            tool.function.as_ref().unwrap().description,
            Some(String::from("test"))
        );
        assert_eq!(
            tool.function
                .as_ref()
                .unwrap()
                .parameters
                .as_ref()
                .unwrap()
                .properties
                .get("test")
                .unwrap()
                .description,
            String::from("test")
        );
    }
}
