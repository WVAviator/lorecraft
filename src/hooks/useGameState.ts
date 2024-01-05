import { invoke } from '@tauri-apps/api';
import React from 'react';
import { GameStateContext } from '../context/GameStateProvider';
import { useNavigate } from 'react-router-dom';

const useGameState = () => {
  const { gameState, setGameState } = React.useContext(GameStateContext);
  const navigate = useNavigate();
  const [loading, setLoading] = React.useState(true);

  const startGame = async (gameId: string) => {
    setLoading(true);
    try {
      const { game_state } = (await invoke('start_game', {
        request: {
          game_id: gameId,
        },
      })) as { game_state: GameState };
      setGameState(game_state);
    } catch (error) {
      console.error('Error starting game: ', error);
      navigate('/mainmenu');
    } finally {
      setLoading(false);
    }
  };

  const sendNarrativeMessage = async (message: string) => {
    if (!gameState) return;
    setLoading(true);

    // Temporarily set the state with new message for immediate feedback
    setGameState((state) => {
      let newState = structuredClone(state);
      newState?.messages.push(`Player: ${message}`);
      return newState;
    });

    try {
      const { game_state } = (await invoke('game_prompt', {
        request: { prompt: message },
      })) as { game_state: GameState };
      setGameState(game_state);
    } catch (error) {
      console.error('Error occurred sending prompt: ', error);
      navigate('/mainmenu');
    } finally {
      setLoading(false);
    }
  };

  return { gameState, startGame, sendNarrativeMessage, loading };
};

export default useGameState;
