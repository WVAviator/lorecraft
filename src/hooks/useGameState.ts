import { invoke } from '@tauri-apps/api';
import React from 'react';
import { GameStateContext } from '../context/GameStateProvider';
import { useNavigate } from 'react-router-dom';
import { appWindow } from '@tauri-apps/api/window';
import { Event, UnlistenFn } from '@tauri-apps/api/event';

const useGameState = () => {
  const { gameState, setGameState } = React.useContext(GameStateContext);
  const navigate = useNavigate();
  const [loading, setLoading] = React.useState(true);

  React.useEffect(() => {
    console.log('Initializing event listener for game state.');
    let unlisten: UnlistenFn;
    const subscribe = async () => {
      unlisten = await appWindow.listen('state', (event: Event<GameState>) => {
        if (!event.payload) return;
        console.log('Setting game state through event update: ', event.payload);
        setGameState(event.payload);
      });
    };
    subscribe();
    return () => {
      unlisten();
    };
  }, [setGameState]);

  const startGame = async (gameId: string) => {
    if (gameState) {
      console.warn(
        'Attempted to start a new game, despite a game already loaded.'
      );
      return;
    }

    console.log('Starting a new game with ID: ', gameId);

    setLoading(true);
    try {
      const { game_state } = (await invoke('start_game', {
        request: {
          game_id: gameId,
        },
      })) as { game_state: GameState };

      console.log(
        'Received game state for new game with initial narrative: ',
        game_state.messages[0]
      );

      setGameState(game_state);
      setLoading(false);
    } catch (error) {
      console.error('Error starting game: ', error);
      navigate('/mainmenu');
    }
  };

  const sendNarrativeMessage = async (message: string) => {
    if (!gameState) {
      console.error(
        "Attempted to send a message to a game that doesn't exist."
      );
    }
    setLoading(true);

    console.log(`Player entered narrative message "${message}"`);

    try {
      const { game_state } = (await invoke('game_prompt', {
        request: { prompt: message },
      })) as { game_state: GameState };

      console.log(
        'Received response from narrator: ',
        game_state.messages[game_state.messages.length - 1]
      );

      setGameState(game_state);
      setLoading(false);
    } catch (error) {
      console.error('Error occurred sending prompt: ', error);
      navigate('/mainmenu');
    }
  };

  const sendCharacterMessage = async (message: string) => {
    if (!gameState) {
      console.error(
        "Attempted to send a character message to a game session that doesn't exist."
      );
    }
    setLoading(true);

    console.log(`Player entered character message "${message}"`);

    try {
      const { game_state } = (await invoke('character_prompt', {
        request: { message },
      })) as { game_state: GameState };
      setGameState(game_state);
      setLoading(false);
    } catch (error) {
      console.error(
        'Failed to receive response from character prompt: ',
        error
      );
      navigate('/mainmenu');
    }
  };

  const characterTradeResponse = async (accept: boolean) => {
    if (!gameState) {
      console.error(
        "Attempted to send a character trade response to a game session that doesn't exist."
      );
    }
    setLoading(true);

    console.log(`Player entered character trade response "${accept}"`);

    try {
      const { game_state } = (await invoke('character_prompt', {
        request: { trade_accept: accept },
      })) as { game_state: GameState };
      setGameState(game_state);
      setLoading(false);
    } catch (error) {
      console.error(
        'Failed to receive response from character prompt: ',
        error
      );
      navigate('/mainmenu');
    }
  };

  const endCharacterConversation = async () => {
    if (!gameState) {
      console.error(
        "Attempted to end a character conversation to a game session that doesn't exist."
      );
    }
    setLoading(true);

    console.log('Player ended character conversation.');

    try {
      const { game_state } = (await invoke('character_prompt', {
        request: { end_conversation: true },
      })) as { game_state: GameState };
      setGameState(game_state);
      setLoading(false);
    } catch (error) {
      console.error(
        'Failed to receive response from character prompt: ',
        error
      );
      navigate('/mainmenu');
    }
  };

  const endGame = async () => {
    if (!gameState) {
      console.error("Attempted to end a game session that doesn't exist.");
    }
    setLoading(true);

    console.log('Player ended the game session.');

    try {
      // await invoke('end_game'); TODO: Implement this is Rust
      setGameState(null);
      setLoading(false);
    } catch (error) {
      console.error('Failed to end game: ', error);
      navigate('/mainmenu');
    }
  };

  return {
    gameState,
    startGame,
    sendNarrativeMessage,
    loading,
    sendCharacterMessage,
    characterTradeResponse,
    endCharacterConversation,
    endGame,
  };
};

export default useGameState;
