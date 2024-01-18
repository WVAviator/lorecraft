use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::{common::Metadata, model::ChatModel, Error};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TypedBuilder)]
#[builder(mutators(
    #[mutator(requires = [metadata])]
    fn add_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key, value);
    }
))]
pub struct CreateRunRequest {
    #[builder(setter(into))]
    assistant_id: String,
    #[builder(default, setter(strip_option))]
    model: Option<ChatModel>,
    #[builder(default, setter(strip_option))]
    instructions: Option<String>,
    #[builder(default, setter(strip_option))]
    additional_instructions: Option<String>,
    #[builder(default, setter(strip_option))]
    tools: Option<Vec<String>>,
    #[builder(default)]
    metadata: Metadata,
}

impl CreateRunRequest {
    pub fn to_json_body(self) -> Result<String, Error> {
        self.validate()?;
        serde_json::to_string(&self).map_err(|e| Error::SerializationFailure(e.into()))
    }

    fn validate(&self) -> Result<(), Error> {
        self.metadata.validate()?;
        Ok(())
    }
}
