use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::{model::ChatModel, tool::Tool, Error};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TypedBuilder)]
#[builder(mutators(
    #[mutator(requires = [metadata])]
    fn add_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }
))]
pub struct CreateAssistantRequest {
    model: ChatModel,
    #[builder(default, setter(strip_option))]
    name: Option<String>,
    #[builder(default, setter(strip_option))]
    description: Option<String>,
    #[builder(default, setter(strip_option))]
    instructions: Option<String>,
    #[builder(default = Vec::new())]
    tools: Vec<Tool>,
    #[builder(default = Vec::new())]
    file_ids: Vec<String>,
    #[builder(default = HashMap::new())]
    metadata: HashMap<String, String>,
}

impl CreateAssistantRequest {
    pub fn to_json_body(self) -> Result<String, Error> {
        self.validate()?;

        serde_json::to_string(&self).map_err(|e| Error::SerializationFailure(e.into()))
    }

    fn validate(&self) -> Result<(), Error> {
        if let Some(name) = &self.name {
            if name.chars().count() > 256 {
                return Err(Error::InvalidRequestField(String::from(
                    "The field 'name' must be 256 characters or less.",
                )));
            }
        }

        if let Some(description) = &self.description {
            if description.chars().count() > 512 {
                return Err(Error::InvalidRequestField(String::from(
                    "The field 'description' must be 512 characters or less.",
                )));
            }
        }

        if let Some(instructions) = &self.instructions {
            if instructions.chars().count() > 32768 {
                return Err(Error::InvalidRequestField(String::from(
                    "The field 'instructions' must be 32768 characters or less.",
                )));
            }
        }

        if self.tools.len() > 128 {
            return Err(Error::InvalidRequestField(String::from(
                "The field 'tools' must have 128 or fewer items.",
            )));
        }

        if self.file_ids.len() > 20 {
            return Err(Error::InvalidRequestField(String::from(
                "The field 'file_ids' must have 20 or fewer items.",
            )));
        }

        if self
            .metadata
            .iter()
            .any(|(key, value)| key.chars().count() > 64 || value.chars().count() > 512)
        {
            return Err(Error::InvalidRequestField(String::from(
                "The field 'metadata' must have keys of 64 characters or less and values of 512 characters or less.",
            )));
        }

        if self.metadata.len() > 16 {
            return Err(Error::InvalidRequestField(String::from(
                "The field 'metadata' must have 16 or fewer items.",
            )));
        }

        Ok(())
    }
}
