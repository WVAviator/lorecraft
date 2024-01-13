use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::{thread::ToolOutput, Error};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TypedBuilder)]
pub struct SubmitToolOutputsRequest {
    #[builder(default = Vec::new(), mutators(
        fn add_tool_output(&mut self, tool_output: ToolOutput) {
            self.tool_outputs.push(tool_output);
        }
    ))]
    tool_outputs: Vec<ToolOutput>,
}

impl SubmitToolOutputsRequest {
    pub fn to_json_body(self) -> Result<String, Error> {
        self.validate()?;
        serde_json::to_string(&self).map_err(|e| Error::SerializationFailure(e.into()))
    }

    fn validate(&self) -> Result<(), Error> {
        if self.tool_outputs.len() == 0 {
            return Err(Error::InvalidRequestField(String::from(
                "At least one tool output must be provided.",
            )));
        }

        Ok(())
    }
}
