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
struct Function {
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

    pub fn from_json(mut self, json: &str) -> Result<Self, Error> {
        self =
            serde_json::from_str::<Function>(json).map_err(|e| Error::DeserializationFailure(e))?;

        Ok(self)
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

        let parameters = self.parameters.unwrap();

        parameters.properties.set(
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
