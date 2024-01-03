use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitToolOutputsRequest {
    pub tool_outputs: Vec<ToolOutput>,
}

impl SubmitToolOutputsRequest {
    pub fn new() -> Self {
        SubmitToolOutputsRequest {
            tool_outputs: vec![],
        }
    }

    pub fn add_output(mut self, tool_call_id: &str, output: &str) -> Self {
        self.tool_outputs.push(ToolOutput {
            tool_call_id: tool_call_id.to_string(),
            output: output.to_string(),
        });
        self
    }

    pub fn to_request_body(self) -> Result<String, anyhow::Error> {
        serde_json::to_string(&self).context("Unable to serialize request body.")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolOutput {
    pub tool_call_id: String,
    pub output: String,
}

// curl https://api.openai.com/v1/threads/thread_abc123/runs/run_123/submit_tool_outputs \
//   -H "Content-Type: application/json" \
//   -H "Authorization: Bearer $OPENAI_API_KEY" \
//   -H "OpenAI-Beta: assistants=v1" \
//   -d '{
//     "tool_outputs": [{
//       "tool_call_id": "call_abc123",
//       "output": "{"temperature": "22", "unit": "celsius"}"
//     }, {
//       "tool_call_id": "call_abc456",
//       "output": "{"nickname": "LA"}"
//     }]
//   }'
