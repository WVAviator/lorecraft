use anyhow::{anyhow, bail};
use log::{debug, info, trace};

use crate::{
    game::Game,
    game_state::GameState,
    openai_client::{list_messages::list_messages_query::ListMessagesQuery, OpenAIClient},
    session_context::session_request::SessionRequest,
};

use super::SessionState;

pub struct CharacterReadMessageState {}

impl CharacterReadMessageState {
    pub async fn process(
        request: SessionRequest,
        openai_client: &OpenAIClient,
        game_state: &mut GameState,
        game: &Game,
    ) -> Result<SessionState, anyhow::Error> {
        match request {
            SessionRequest::ContinueProcessing => {
                let thread_id = game_state
                    .character_interaction
                    .as_ref()
                    .ok_or(anyhow!("No character interaction to read messages from."))?
                    .thread_id
                    .clone();
                let character_id = game_state
                    .character_interaction
                    .as_ref()
                    .ok_or(anyhow!("No character interaction to read messages from."))?
                    .character_id
                    .clone();
                let character_name = &game
                    .characters
                    .iter()
                    .find(|c| c.id.eq(&character_id))
                    .ok_or(anyhow!("Could not find character."))?
                    .name;

                info!("Fetching latest message in thread.");
                let message_list_response = openai_client
                    .list_messages(
                        ListMessagesQuery::builder(&thread_id)
                            .limit(1)
                            .order("desc")
                            .build(),
                    )
                    .await
                    .map_err(|e| {
                        anyhow!("Unable to retrieve latest message from thread:\n{:?}", e)
                    })?;
                trace!(
                    "Received latest message from thread:\n{:?}",
                    &message_list_response
                );

                info!("Processing message meta-commands");
                let character_message = message_list_response.data[0].content[0].text.value.clone();

                let closed = game_state
                    .character_interaction
                    .as_ref()
                    .ok_or(anyhow!("Unable to access character interaction."))?
                    .closed
                    .clone();

                if closed {
                    return Ok(SessionState::CharacterEndInteractionState {
                        summary: Some(character_message),
                    });
                }

                let processed_messages = CharacterReadMessageState::process_meta_commands(
                    &character_name,
                    character_message,
                );

                info!("Appending new messages to game state.");
                for message in processed_messages {
                    if message.starts_with(format!("{}:", character_name).as_str()) {
                        game_state
                            .character_interaction
                            .as_mut()
                            .ok_or(anyhow!("No character interaction to append message to."))?
                            .add_message(&message);
                    } else {
                        game_state
                            .character_interaction
                            .as_mut()
                            .ok_or(anyhow!(
                                "No character interaction to append nonverbal action to."
                            ))?
                            .add_nonverbal(&message);
                    }
                }
                Ok(SessionState::CharacterIdleState)
            }
            _ => bail!(
                "Unexpected request received for CharacterReadMessageState: {:?}.",
                request
            ),
        }
    }

    fn process_meta_commands(character_name: &str, message: String) -> Vec<String> {
        let mut messages = Vec::new();

        debug!("Processing meta-commands for string: {}", &message);

        let message = {
            // Sometimes the LLM returns the character name prefix, sometimes it does not
            if message.starts_with(format!("{}:", character_name).as_str()) {
                message.split(":").skip(1).collect::<String>()
            } else {
                message
            }
        };

        debug!("Modified initial string: {}", &message);

        let mut current_message = Vec::new();
        for word in message.split(" ") {
            if word.starts_with("$emotion") {
                let new_message = format!("{}: {}", character_name, current_message.join(" "));

                debug!("Extracted message: {}", &new_message);
                messages.push(new_message);

                current_message.clear();

                let emotion: String = word
                    .chars()
                    .skip(9)
                    .take(word.chars().count() - 10)
                    .collect();
                let emotion_message = format!("{} is feeling {}", character_name, emotion);

                debug!("Extracted nonverbal: {}", &emotion_message);
                messages.push(emotion_message);
            } else if word.starts_with("$action") {
                let new_message = format!("{}: {}", character_name, current_message.join(" "));

                debug!("Extracted message: {}", &new_message);
                messages.push(new_message);

                current_message.clear();

                let action: String = word
                    .chars()
                    .skip(8)
                    .take(word.chars().count() - 9)
                    .collect();
                let action_message = format!("{} {}", character_name, action);

                debug!("Extracted nonverbal: {}", &action_message);
                messages.push(action_message);
            } else {
                current_message.push(word);
            }
        }

        if !current_message.is_empty() {
            let new_message = format!("{}: {}", character_name, current_message.join(" "));

            debug!("Extracted message: {}", &new_message);
            messages.push(new_message);
        }

        debug!("Finished. Returning messages: {:?}", messages);

        messages
    }
}
