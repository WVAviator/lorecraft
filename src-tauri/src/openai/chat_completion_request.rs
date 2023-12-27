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

    pub fn request_body(&self) -> String {
        json!({
            "model": model.to_string(),
            "messages": [
                {
                    "role": "system",
                    "content": system_prompt
                },
                {
                    "role": "user",
                    "content": user_prompt
                }
            ]
        })
    }
}
