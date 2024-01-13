use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::{common::Metadata, model::ChatModel, tool::Tool, Error};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TypedBuilder)]
#[builder(mutators(
    #[mutator(requires = [metadata])]
    fn add_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key, value);
    }
))]
pub struct CreateAssistantRequest {
    model: ChatModel,
    #[builder(default, setter(strip_option, into))]
    name: Option<String>,
    #[builder(default, setter(strip_option, into))]
    description: Option<String>,
    #[builder(default, setter(strip_option, into))]
    instructions: Option<String>,
    #[builder(default = Vec::new(), mutators(
        pub fn add_tool(&mut self, tool: Tool) {
            self.tools.push(tool);
        }
    ), via_mutators)]
    tools: Vec<Tool>,
    #[builder(default = Vec::new())]
    file_ids: Vec<String>,
    #[builder(default)]
    metadata: Metadata,
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

        self.metadata.validate()?;

        Ok(())
    }
}
