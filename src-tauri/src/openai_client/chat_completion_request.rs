pub struct ChatCompletionRequest {
    system_prompt: String,
    user_prompt: String,
    model: ChatCompletionModel,
}

impl ChatCompletionRequest {
    pub fn new(system_prompt: String, user_prompt: String, mode: OpenAIModel) -> Self {
        ChatCompletionRequest {
            system_prompt,
            user_prompt,
            model,
        }
    }

    pub fn to_request_body(self) -> String {
        json!({
            "model": self.model,
            "messages": [
                {
                    "role": "system",
                    "content": self.system_prompt
                },
                {
                    "role": "user",
                    "content": self.user_prompt
                }
            ]
        }).to_string()
    }
}
