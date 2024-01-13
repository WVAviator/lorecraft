use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::{common::Metadata, Error};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TypedBuilder)]
#[builder(mutators(
    #[mutator(requires = [file_ids])]
    fn add_file_id(&mut self, file_id: impl Into<String>) {
        self.file_ids.push(file_id.into());
    }
    #[mutator(requires = [metadata])]
    fn add_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key, value);
    }
))]
pub struct CreateMessageRequest {
    #[builder(default = String::from("user"), setter(skip))]
    role: String,
    #[builder(setter(into))]
    content: String,
    #[builder(default = Vec::new())]
    file_ids: Vec<String>,
    #[builder(default)]
    metadata: Metadata,
}

impl CreateMessageRequest {
    pub fn to_json_body(self) -> Result<String, Error> {
        self.validate()?;
        serde_json::to_string(&self).map_err(|e| Error::SerializationFailure(e.into()))
    }

    fn validate(&self) -> Result<(), Error> {
        self.metadata.validate()?;

        if self.file_ids.len() > 10 {
            return Err(Error::InvalidRequestField(String::from(
                "The field 'file_ids' must have 10 or fewer items.",
            )));
        }

        if self.role.ne("user") {
            return Err(Error::InvalidRequestField(String::from(
                "The field 'role' must be 'user'.",
            )));
        }

        Ok(())
    }
}
