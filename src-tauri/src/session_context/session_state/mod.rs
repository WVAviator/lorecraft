use anyhow::Context;

use crate::{
    game_state::GameState,
    openai_client::{retrieve_run::retrieve_run_response::ToolCall, OpenAIClient},
};

use self::{
    idle_state::IdleState, pending_run_state::PendingRunState, polling_run_state::PollingRunState,
    read_message_state::ReadMessageState, requires_action_state::RequiresActionState,
};

use super::session_request::SessionRequest;

mod idle_state;
mod pending_run_state;
mod polling_run_state;
mod read_message_state;
mod requires_action_state;

#[derive(Debug)]
pub enum SessionState {
    IdleState,
    PendingRunState,
    PollingRunState {
        run_id: String,
    },
    RequiresActionState {
        run_id: String,
        tool_call: ToolCall,
    },
    ReadMessageState,
    ProcessNewSceneState {
        run_id: String,
        tool_call_id: String,
        arguments: serde_json::Value,
    },
    ProcessAddItemState {
        run_id: String,
        tool_call_id: String,
        arguments: serde_json::Value,
    },
    ProcessRemoveItemState {
        run_id: String,
        tool_call_id: String,
        arguments: serde_json::Value,
    },
    ProcessCharacterInteractState {
        run_id: String,
        tool_call_id: String,
        arguments: serde_json::Value,
    },
    ProcessEndGameState {
        run_id: String,
        tool_call_id: String,
        arguments: serde_json::Value,
    },
}

impl SessionState {
    pub async fn process(
        self,
        request: SessionRequest,
        openai_client: &OpenAIClient,
        game_state: &mut GameState,
    ) -> Result<SessionState, anyhow::Error> {
        match self {
            SessionState::IdleState => IdleState::process(request, openai_client, game_state)
                .await
                .context("Failed to process state change from IdleState."),
            SessionState::PendingRunState => {
                PendingRunState::process(request, openai_client, game_state)
                    .await
                    .context("Failed to process state change from PendingRunState")
            }
            SessionState::PollingRunState { run_id } => {
                PollingRunState::process(request, openai_client, game_state, run_id)
                    .await
                    .context("Failed to process state change from PollingRunState")
            }
            SessionState::RequiresActionState { run_id, tool_call } => {
                RequiresActionState::process(request, openai_client, game_state, run_id, tool_call)
                    .await
                    .context("Failed to process state change from RequiresActionState.")
            }
            SessionState::ReadMessageState => {
                ReadMessageState::process(request, openai_client, game_state)
                    .await
                    .context("Failed to process state change from ReadMessageState.")
            }
            SessionState::ProcessNewSceneState {
                run_id,
                tool_call_id,
                arguments,
            } => todo!(),
            SessionState::ProcessAddItemState {
                run_id,
                tool_call_id,
                arguments,
            } => todo!(),
            SessionState::ProcessRemoveItemState {
                run_id,
                tool_call_id,
                arguments,
            } => todo!(),
            SessionState::ProcessCharacterInteractState {
                run_id,
                tool_call_id,
                arguments,
            } => todo!(),
            SessionState::ProcessEndGameState {
                run_id,
                tool_call_id,
                arguments,
            } => todo!(),
        }
    }

    pub fn should_continue_processing(&self) -> bool {
        match self {
            SessionState::IdleState => false,
            SessionState::PendingRunState => true,
            SessionState::PollingRunState { run_id } => true,
            SessionState::RequiresActionState { run_id, tool_call } => true,
            SessionState::ReadMessageState => true,
            SessionState::ProcessNewSceneState {
                run_id,
                tool_call_id,
                arguments,
            } => todo!(),
            SessionState::ProcessAddItemState {
                run_id,
                tool_call_id,
                arguments,
            } => todo!(),
            SessionState::ProcessRemoveItemState {
                run_id,
                tool_call_id,
                arguments,
            } => todo!(),
            SessionState::ProcessCharacterInteractState {
                run_id,
                tool_call_id,
                arguments,
            } => todo!(),
            SessionState::ProcessEndGameState {
                run_id,
                tool_call_id,
                arguments,
            } => todo!(),
        }
    }
}
