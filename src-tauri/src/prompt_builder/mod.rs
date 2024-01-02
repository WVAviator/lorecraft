use std::fs;

pub struct PromptBuilder {
    prompt: String,
}

impl PromptBuilder {
    pub fn new() -> Self {
        Self {
            prompt: String::new(),
        }
    }

    pub fn add_plain_text(&mut self, prompt: &str) -> &mut Self {
        self.prompt += prompt;
        self.prompt += "\n";
        self
    }

    pub fn add_prompt(&mut self, path: &str) -> &mut Self {
        self.prompt +=
            &fs::read_to_string(path).expect("Something went wrong reading the prompt file.");
        self.prompt += "\n";
        self
    }

    pub fn add_example_input(&mut self, path: &str) -> &mut Self {
        self.prompt += "\nExample Input:\n";
        self.prompt +=
            &fs::read_to_string(path).expect("Something went wrong reading the example file.");
        self.prompt += "\n";
        self
    }

    pub fn add_example_output(&mut self, path: &str) -> &mut Self {
        self.prompt += "\nExample Output:\n";
        self.prompt +=
            &fs::read_to_string(path).expect("Something went wrong reading the example file.");
        self.prompt += "\n";
        self
    }

    pub fn build(&self) -> String {
        self.prompt.clone()
    }
}
