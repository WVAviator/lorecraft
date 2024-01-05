import { invoke } from '@tauri-apps/api';
import React from 'react';
import { GameStateContext } from '../context/GameStateProvider';
import { useNavigate } from 'react-router-dom';

const useGameState = () => {
  const { gameState, setGameState } = React.useContext(GameStateContext);
  const navigate = useNavigate();
  const [loading, setLoading] = React.useState(true);
  const [started, setStarted] = React.useState(false);

  const startGame = async (gameId: string) => {
    if (started) {
      console.warn('Attempted to start a game that has already been started.');
    }
    setStarted(true);
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
    if (!gameState || !started) {
      console.error(
        "Attempted to send a message to a game that either doesn't exist or hasn't been started yet."
      );
    }
    setLoading(true);

    console.log(`Player entered narrative message "${message}"`);

    // Temporarily set the state with new message for immediate feedback
    setGameState((state) => {
      let newState = structuredClone(state);
      newState?.messages.push(`> ${message}`);
      return newState;
    });

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

  return { gameState, startGame, sendNarrativeMessage, loading };
};

export default useGameState;
