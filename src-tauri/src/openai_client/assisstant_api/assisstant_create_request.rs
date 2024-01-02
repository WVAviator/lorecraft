use serde::{Deserialize, Serialize};

use crate::openai_client::chat_completion::chat_completion_model::ChatCompletionModel;

use super::assisstant_tool::AssisstantTool;

#[derive(Debug, Serialize, Deserialize)]
pub struct AssisstantCreateRequest {
    pub instructions: String,
    pub name: String,
    pub tools: Vec<AssisstantTool>,
    pub model: String,
}

impl AssisstantCreateRequest {
    pub fn new(instructions: String, name: String, model: ChatCompletionModel) -> Self {
        Self {
            instructions,
            name,
            tools: vec![],
            model: model.to_string(),
        }
    }

    pub fn to_request_body(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

// curl "https://api.openai.com/v1/assistants" \
//   -H "Content-Type: application/json" \
//   -H "Authorization: Bearer $OPENAI_API_KEY" \
//   -H "OpenAI-Beta: assistants=v1" \
//   -d '{
//     "instructions": "You are a personal math tutor. When asked a question, write and run Python code to answer the question.",
//     "name": "Math Tutor",
//     "tools": [{"type": "code_interpreter"}],
//     "model": "gpt-4"
//   }'
