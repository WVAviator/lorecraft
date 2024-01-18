use anyhow::{anyhow, bail};
use log::{debug, info, trace};
use openai_lib::{
    message::{ListMessagesRequest, MessageClient, MessageSortOrder},
    OpenAIClient,
};

use crate::{game::Game, game_state::GameState, session_context::session_request::SessionRequest};

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
                let character_name = game_state
                    .character_interaction
                    .as_ref()
                    .ok_or(anyhow!("No character interaction to read messages from."))?
                    .character_name
                    .clone();
                let character_name = &game
                    .characters
                    .iter()
                    .find(|c| c.name.eq(&character_name))
                    .ok_or(anyhow!("Could not find character."))?
                    .name;

                info!("Fetching latest message in thread.");
                let message_list_response = openai_client
                    .list_messages(
                        ListMessagesRequest::builder()
                            .limit(1)
                            .order(MessageSortOrder::Descending)
                            .build(),
                        &thread_id,
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
                let character_message = message_list_response
                    .data
                    .get(0)
                    .ok_or(anyhow!("No messages returned."))?
                    .get_text_content();

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

    // fn process_meta_commands(character_name: &str, message: String) -> Vec<String> {
    //     let mut messages = Vec::new();

    //     debug!("Processing meta-commands for string: {}", &message);

    //     let message = {
    //         // Sometimes the LLM returns the character name prefix, sometimes it does not
    //         if message.starts_with(format!("{}:", character_name).as_str()) {
    //             message.split(":").skip(1).collect::<String>()
    //         } else {
    //             message
    //         }
    //     };

    //     debug!("Modified initial string: {}", &message);

    //     let mut current_message = Vec::new();
    //     for word in message.split(" ") {
    //         if word.starts_with("$emotion") {
    //             // Save the current message as we've hit a meta command
    //             if current_message.len() > 0 {
    //                 let new_message = format!("{}: {}", character_name, current_message.join(" "));
    //                 debug!("Extracted message: {}", &new_message);
    //                 messages.push(new_message);
    //                 current_message.clear();
    //             }

    //             let emotion: String = word
    //                 .chars()
    //                 .skip(9)
    //                 .take(word.chars().count() - 10)
    //                 .collect();
    //             let emotion_message = format!("{} is feeling {}", character_name, emotion);

    //             debug!("Extracted nonverbal: {}", &emotion_message);
    //             messages.push(emotion_message);
    //         } else if word.starts_with("$action") {
    //             let new_message = format!("{}: {}", character_name, current_message.join(" "));

    //             debug!("Extracted message: {}", &new_message);
    //             messages.push(new_message);

    //             current_message.clear();

    //             let action: String = word
    //                 .chars()
    //                 .skip(8)
    //                 .take(word.chars().count() - 9)
    //                 .collect();
    //             let action_message = format!("{} {}", character_name, action);

    //             debug!("Extracted nonverbal: {}", &action_message);
    //             messages.push(action_message);
    //         } else {
    //             current_message.push(word);
    //         }
    //     }

    //     if !current_message.is_empty() {
    //         let new_message = format!("{}: {}", character_name, current_message.join(" "));

    //         debug!("Extracted message: {}", &new_message);
    //         messages.push(new_message);
    //     }

    //     debug!("Finished. Returning messages: {:?}", messages);

    //     messages
    // }

    fn process_meta_commands(character_name: &str, message: String) -> Vec<String> {
        let mut messages = Vec::new();

        debug!("Processing meta-commands for string: {}", &message);

        let message = {
            // Sometimes the LLM returns the character name prefix, sometimes it does not
            if message.starts_with(format!("{}:", character_name).as_str()) {
                message
                    .split(":")
                    .skip(1)
                    .collect::<String>()
                    .trim()
                    .to_string()
            } else {
                message.trim().to_string()
            }
        };

        debug!("Modified initial string: {}", &message);

        enum ProcessingState {
            CollectingMessage,
            CollectingEmotion,
            CollectingAction,
        }

        let mut processing_state = ProcessingState::CollectingMessage;

        let mut collection = Vec::new();
        for word in message.split(" ") {
            match processing_state {
                ProcessingState::CollectingMessage => match word {
                    word if word.starts_with("$emotion") => {
                        if collection.len() > 0 {
                            messages.push(format!("{}: {}", character_name, collection.join(" ")));
                            collection.clear();
                        }
                        if word.ends_with(")") {
                            // Single word emotion
                            let modified_word = word
                                .chars()
                                .skip(9)
                                .take(word.chars().count() - 10)
                                .collect::<String>();
                            messages
                                .push(format!("{} is feeling {}", character_name, modified_word));
                        } else {
                            // Multi-word emotion
                            let modified_word = word.chars().skip(9).collect::<String>();

                            collection.push(modified_word);
                            processing_state = ProcessingState::CollectingEmotion;
                        }
                    }
                    word if word.starts_with("$action") => {
                        if collection.len() > 0 {
                            messages.push(format!("{}: {}", character_name, collection.join(" ")));
                            collection.clear();
                        }
                        if word.ends_with(")") {
                            // Single word action
                            let modified_word = word
                                .chars()
                                .skip(8)
                                .take(word.chars().count() - 9)
                                .collect::<String>();
                            messages.push(format!("{} {}", character_name, modified_word));
                        } else {
                            // Multi-word action
                            let modified_word = word.chars().skip(8).collect::<String>();

                            collection.push(modified_word);
                            processing_state = ProcessingState::CollectingAction;
                        }
                    }
                    _ => collection.push(word.trim().to_string()),
                },
                ProcessingState::CollectingEmotion => match word {
                    word if word.ends_with(")") => {
                        let modified_word = word
                            .chars()
                            .take(word.chars().count() - 1)
                            .collect::<String>();
                        collection.push(modified_word);
                        messages.push(format!(
                            "{} is feeling {}",
                            character_name,
                            collection.join(" ")
                        ));
                        collection.clear();
                        processing_state = ProcessingState::CollectingMessage;
                    }
                    _ => collection.push(word.trim().to_string()),
                },
                ProcessingState::CollectingAction => match word {
                    word if word.ends_with(")") => {
                        let modified_word = word
                            .chars()
                            .take(word.chars().count() - 1)
                            .collect::<String>();
                        collection.push(modified_word);
                        messages.push(format!("{} {}", character_name, collection.join(" ")));
                        collection.clear();
                        processing_state = ProcessingState::CollectingMessage;
                    }
                    _ => collection.push(word.trim().to_string()),
                },
            }
        }

        if !collection.is_empty() {
            messages.push(format!("{}: {}", character_name, collection.join(" ")));
        }

        return messages;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn meta_commands_processed_properly() {
        let message = String::from("$emotion(sad) I am deeply saddened by this news.");
        let processed_messages = CharacterReadMessageState::process_meta_commands("Bob", message);
        assert_eq!(processed_messages.len(), 2);
        assert_eq!(processed_messages.get(0).unwrap(), "Bob is feeling sad");
        assert_eq!(
            processed_messages.get(1).unwrap(),
            "Bob: I am deeply saddened by this news."
        );
    }

    #[test]
    fn meta_commands_work_even_when_actor_prefixes() {
        let message = String::from("Bob: $emotion(sad) I am deeply saddened by this news.");
        let processed_messages = CharacterReadMessageState::process_meta_commands("Bob", message);
        assert_eq!(processed_messages.len(), 2);
        assert_eq!(processed_messages.get(0).unwrap(), "Bob is feeling sad");
        assert_eq!(
            processed_messages.get(1).unwrap(),
            "Bob: I am deeply saddened by this news."
        );
    }

    #[test]
    fn handles_multiple_embedded_meta_commands() {
        let message = String::from("Bob: Hello! $emotion(excited) How are you? $action(points behind you) Looks like a storm is coming...");
        let processed_messages = CharacterReadMessageState::process_meta_commands("Bob", message);
        let expected: Vec<String> = vec![
            "Bob: Hello!".into(),
            "Bob is feeling excited".into(),
            "Bob: How are you?".into(),
            "Bob points behind you".into(),
            "Bob: Looks like a storm is coming...".into(),
        ];
        assert_eq!(processed_messages.len(), expected.len());
        assert_eq!(processed_messages, expected);
    }

    #[test]
    fn handles_multiword_actions_and_emotions() {
        let message = String::from("Bob: $emotion(very excited) I am very excited to see you! $action(looks at you) How are you?");
        let processed_messages = CharacterReadMessageState::process_meta_commands("Bob", message);
        let expected: Vec<String> = vec![
            "Bob is feeling very excited".into(),
            "Bob: I am very excited to see you!".into(),
            "Bob looks at you".into(),
            "Bob: How are you?".into(),
        ];
        assert_eq!(processed_messages.len(), expected.len());
        assert_eq!(processed_messages, expected);
    }
}
