use anyhow::{anyhow, bail};
use log::info;
use openai_lib::{
    message::{ListMessagesRequest, MessageClient, MessageSortOrder},
    OpenAIClient,
};

use crate::{
    game_state::GameState,
    session_context::{session_request::SessionRequest, session_state::SessionState},
    utils::string_utilities::StringUtilities,
};

pub struct ReadMessageState {}

impl ReadMessageState {
    pub async fn process(
        session_request: SessionRequest,
        openai_client: &OpenAIClient,
        game_state: &mut GameState,
    ) -> Result<SessionState, anyhow::Error> {
        match session_request {
            SessionRequest::ContinueProcessing => {
                info!("Fetching latest message from thread");
                let list_messages_response = openai_client
                    .list_messages(
                        ListMessagesRequest::builder()
                            .limit(1)
                            .order(MessageSortOrder::Descending)
                            .build(),
                        &game_state.thread_id,
                    )
                    .await
                    .map_err(|e| anyhow!("Failed to list messages: {:?}", e))?;
                // let response = list_messages_response.data[0].content[0].text.value.clone();
                let response = list_messages_response
                    .data
                    .get(0)
                    .ok_or(anyhow!("No messages returned from thread."))?
                    .get_text_content();

                info!(
                    "Received latest response from thread: '{}'",
                    StringUtilities::truncate(&response, 20)
                );

                game_state.add_narrator_message(&response);

                Ok(SessionState::IdleState)
            }
            _ => bail!(
                "Received invalid request type {:?}. Expected ContinueProcessing.",
                &session_request
            ),
        }
    }
}
