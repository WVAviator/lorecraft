use anyhow::Context;
use openai_lib::{tool::ToolCall, OpenAIClient};

use crate::{game::Game, game_state::GameState};

use self::{
    awaiting_player_gift_response_state::AwaitingPlayerGiftResponseState,
    awaiting_player_trade_response_state::AwaitingPlayerTradeResponseState,
    character_end_interaction_state::CharacterEndInteractionState,
    character_idle_state::CharacterIdleState,
    character_polling_run_state::CharacterPollingRunState,
    character_read_message_state::CharacterReadMessageState,
    character_requires_action_state::CharacterRequiresActionState,
    character_run_request_state::CharacterRunRequestState, idle_state::IdleState,
    pending_run_state::PendingRunState, polling_run_state::PollingRunState,
    process_add_item_state::ProcessAddItemState,
    process_character_gift_state::ProcessCharacterGiftState,
    process_character_interact_state::ProcessCharacterInteractState,
    process_character_trade_state::ProcessCharacterTradeState,
    process_end_game::ProcessEndGameState, process_new_scene_state::ProcessNewSceneState,
    process_remove_item_state::ProcessRemoveItemState, read_message_state::ReadMessageState,
    requires_action_state::RequiresActionState, submit_tool_outputs_state::SubmitToolOutputsState,
};

use super::session_request::SessionRequest;

mod awaiting_player_gift_response_state;
mod awaiting_player_trade_response_state;
mod character_end_interaction_state;
mod character_idle_state;
mod character_polling_run_state;
mod character_read_message_state;
mod character_requires_action_state;
mod character_run_request_state;
mod idle_state;
mod pending_run_state;
mod polling_run_state;
mod process_add_item_state;
mod process_character_gift_state;
mod process_character_interact_state;
mod process_character_trade_state;
mod process_end_game;
mod process_new_scene_state;
mod process_remove_item_state;
mod read_message_state;
mod requires_action_state;
mod submit_tool_outputs_state;

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
    SubmitToolOutputsState {
        run_id: String,
        tool_call_id: String,
        output: String,
    },
    CharacterRunRequestState,
    CharacterPollingRunState {
        run_id: String,
    },
    CharacterRequiresActionState {
        run_id: String,
        tool_call: ToolCall,
    },
    ProcessCharacterTradeState {
        run_id: String,
        tool_call_id: String,
        arguments: serde_json::Value,
    },
    ProcessCharacterGiftState {
        run_id: String,
        tool_call_id: String,
        arguments: serde_json::Value,
    },
    AwaitingPlayerTradeResponseState {
        run_id: String,
        tool_call_id: String,
    },
    AwaitingPlayerGiftResponseState {
        run_id: String,
        tool_call_id: String,
    },
    CharacterReadMessageState,
    CharacterIdleState,
    CharacterEndInteractionState {
        summary: Option<String>,
    },
}

impl SessionState {
    pub async fn process(
        self,
        request: SessionRequest,
        openai_client: &OpenAIClient,
        game_state: &mut GameState,
        game: &Game,
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
                RequiresActionState::process(request, run_id, tool_call)
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
            } => ProcessNewSceneState::process(
                request,
                game_state,
                run_id,
                tool_call_id,
                arguments,
                game,
            )
            .await
            .context("Failed to process state change from ProcessNewSceneState."),
            SessionState::ProcessAddItemState {
                run_id,
                tool_call_id,
                arguments,
            } => ProcessAddItemState::process(request, game_state, run_id, tool_call_id, arguments)
                .await
                .context("Failed to process state change from ProcessAddItemState."),
            SessionState::ProcessRemoveItemState {
                run_id,
                tool_call_id,
                arguments,
            } => ProcessRemoveItemState::process(
                request,
                game_state,
                run_id,
                tool_call_id,
                arguments,
            )
            .await
            .context("Failed to process state change from ProcessRemoveItemState."),
            SessionState::ProcessCharacterInteractState {
                run_id,
                tool_call_id,
                arguments,
            } => ProcessCharacterInteractState::process(
                request,
                openai_client,
                game_state,
                run_id,
                tool_call_id,
                arguments,
                game,
            )
            .await
            .context("Failed to process state change from ProcessCharacterInteractState"),
            SessionState::ProcessEndGameState {
                run_id,
                tool_call_id,
                arguments,
            } => ProcessEndGameState::process(request, game_state, run_id, tool_call_id, arguments)
                .await
                .context("Failed to process state change from ProcessEndGameState."),
            SessionState::SubmitToolOutputsState {
                run_id,
                tool_call_id,
                output,
            } => SubmitToolOutputsState::process(
                request,
                openai_client,
                game_state,
                run_id,
                tool_call_id,
                output,
            )
            .await
            .context("Failed to process state change from SubmitToolOutputsState."),
            SessionState::CharacterRunRequestState => {
                CharacterRunRequestState::process(request, openai_client, game_state)
                    .await
                    .context("Failed to process state change from CharacterRunRequestState.")
            }
            SessionState::CharacterPollingRunState { run_id } => {
                CharacterPollingRunState::process(request, openai_client, game_state, run_id)
                    .await
                    .context("Failed to process state change from CharacterPollingRunState.")
            }
            SessionState::CharacterRequiresActionState { run_id, tool_call } => {
                CharacterRequiresActionState::process(request, run_id, tool_call)
                    .await
                    .context("Failed to process state change from CharacterRequiresActionState.")
            }
            SessionState::ProcessCharacterTradeState {
                run_id,
                tool_call_id,
                arguments,
            } => ProcessCharacterTradeState::process(
                request,
                game_state,
                run_id,
                tool_call_id,
                arguments,
            )
            .await
            .context("Failed to process state change from ProcessCharacterTradeState."),
            SessionState::ProcessCharacterGiftState {
                run_id,
                tool_call_id,
                arguments,
            } => ProcessCharacterGiftState::process(
                request,
                game_state,
                run_id,
                tool_call_id,
                arguments,
            )
            .await
            .context("Failed to process state change from ProcessCharacterGiftState."),
            SessionState::AwaitingPlayerTradeResponseState {
                run_id,
                tool_call_id,
            } => AwaitingPlayerTradeResponseState::process(
                request,
                openai_client,
                game_state,
                run_id,
                tool_call_id,
            )
            .await
            .context("Failed to process state change from AwaitingPlayerTradeResponseState."),
            SessionState::AwaitingPlayerGiftResponseState {
                run_id,
                tool_call_id,
            } => AwaitingPlayerGiftResponseState::process(
                request,
                openai_client,
                game_state,
                run_id,
                tool_call_id,
            )
            .await
            .context("Failed to process state change from AwaitingPlayerGiftResponseState."),
            SessionState::CharacterReadMessageState => {
                CharacterReadMessageState::process(request, openai_client, game_state, game)
                    .await
                    .context("Failed to process state change from CharacterReadMessageState.")
            }
            SessionState::CharacterIdleState => {
                CharacterIdleState::process(request, openai_client, game_state)
                    .await
                    .context("Failed to process state change from CharacterIdleState.")
            }
            SessionState::CharacterEndInteractionState { summary } => {
                CharacterEndInteractionState::process(request, openai_client, game_state, summary)
                    .await
                    .context("Failed to process state change from CharacterEndInteractionState.")
            }
        }
    }

    pub fn should_continue_processing(&self) -> bool {
        match self {
            SessionState::IdleState => false,
            SessionState::AwaitingPlayerTradeResponseState {
                run_id: _,
                tool_call_id: _,
            } => false,
            SessionState::AwaitingPlayerGiftResponseState {
                run_id: _,
                tool_call_id: _,
            } => false,
            SessionState::CharacterIdleState => false,
            _ => true,
        }
    }
}
