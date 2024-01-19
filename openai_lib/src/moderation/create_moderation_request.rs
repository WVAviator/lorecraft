use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::Error;

use super::ModerationModel;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TypedBuilder)]
pub struct CreateModerationRequest {
    #[builder(setter(into))]
    input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    model: Option<ModerationModel>,
}

impl CreateModerationRequest {
    pub fn to_json_body(self) -> Result<String, Error> {
        serde_json::to_string(&self).map_err(|e| Error::SerializationFailure(e.into()))
    }
}
