use anyhow::anyhow;

use super::character_interaction::CharacterInteraction;

pub struct CharacterInteractionBuilder {
    pub character_name: Option<String>,
    pub assistant_id: Option<String>,
    pub thread_id: Option<String>,
    pub initiating_run_id: Option<String>,
    pub initiating_tool_call_id: Option<String>,
}

impl CharacterInteractionBuilder {
    pub fn new() -> Self {
        CharacterInteractionBuilder {
            character_name: None,
            assistant_id: None,
            thread_id: None,
            initiating_run_id: None,
            initiating_tool_call_id: None,
        }
    }

    pub fn character_name(mut self, character_name: &str) -> Self {
        self.character_name = Some(character_name.to_string());
        self
    }

    pub fn assistant_id(mut self, assistant_id: &str) -> Self {
        self.assistant_id = Some(assistant_id.to_string());
        self
    }

    pub fn thread_id(mut self, thread_id: &str) -> Self {
        self.thread_id = Some(thread_id.to_string());
        self
    }

    pub fn initiating_run_id(mut self, initiating_run_id: &str) -> Self {
        self.initiating_run_id = Some(initiating_run_id.to_string());
        self
    }

    pub fn initiating_tool_call_id(mut self, initiating_tool_call_id: &str) -> Self {
        self.initiating_tool_call_id = Some(initiating_tool_call_id.to_string());
        self
    }

    pub fn build(self) -> Result<CharacterInteraction, anyhow::Error> {
        Ok(CharacterInteraction {
            character_name: self.character_name.ok_or(anyhow!(
                "Cannot start character interaction without character id."
            ))?,
            assistant_id: self.assistant_id.ok_or(anyhow!(
                "Cannot start character interaction without assistant id."
            ))?,
            thread_id: self.thread_id.ok_or(anyhow!(
                "Cannot start character interaction without thread id."
            ))?,
            initiating_run_id: self.initiating_run_id.ok_or(anyhow!(
                "Cannot start character interaction without initiating run id."
            ))?,
            initiating_tool_call_id: self.initiating_tool_call_id.ok_or(anyhow!(
                "Cannot start character interaction without initiating tool call id."
            ))?,
            messages: Vec::new(),
            trade: None,
            closed: false,
        })
    }
}
