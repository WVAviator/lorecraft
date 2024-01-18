import { invoke } from '@tauri-apps/api';
import React from 'react';
import useUpdates from '../hooks/useUpdates';
import CycledBackground from '../components/CycledBackground/CycledBackground';
import { LOADING_IMAGES } from '../data/LoadingImages';
import LoadingSpinner from '../components/LoadingSpinner/LoadingSpinner';
import useGameContext from '../hooks/useGameContext';
import {
  CreateNewGameRequest,
  CreateNewGameResponse,
} from '../types/CreateNewGame';
import { useLocation, useNavigate } from 'react-router-dom';
import AlertDialog from '../components/AlertDialog/AlertDialog';
import { isSetupResponse } from '../types/Setup';

interface GameGenerationScreenProps {}

const GameGenerationScreen: React.FC<GameGenerationScreenProps> = () => {
  const { setGame } = useGameContext();
  const { updates, gameId } = useUpdates();
  const navigate = useNavigate();
  const location = useLocation();

  const [error, setError] = React.useState<string | null>(null);

  React.useEffect(() => {
    const createGame = async () => {
      if (!location.state.request) {
        console.error('No request found in state.');
        navigate('/mainmenu');
      }
      console.log(
        "Calling 'create_new_game' with request: ",
        location.state.request
      );
      try {
        const response = (await invoke('create_new_game', {
          request: location.state.request,
        })) as CreateNewGameResponse;

        setGame(response.game ?? null);
        navigate('/gamemenu');
      } catch (e) {
        console.error('Failed to generate game.');
        if (isSetupResponse(e)) {
          setError(e.message ?? 'Unknown error occurred.');
        }
      }
    };
    createGame();
  }, []);

  const errorActions = [
    {
      title: 'Back to Main Menu',
      onSelect: () => {
        navigate('/mainmenu');
      },
    },
  ];

  if (gameId) {
    errorActions.push({
      title: 'Try Again',
      onSelect: () => {
        navigate('/generate-game', {
          state: {
            request: { prompt: '', resume_previous: gameId },
          },
        });
      },
    });
  }

  return (
    <CycledBackground images={LOADING_IMAGES} play={!error}>
      <div className="flex h-full w-full items-end p-2">
        <div className="absolute bottom-0 left-0 right-0 z-10 flex h-24 items-center gap-4 bg-black p-2">
          <LoadingSpinner />
          <div className="relative flex h-full flex-col-reverse overflow-hidden">
            <div className="absolute left-0 right-0 top-0 h-6 bg-gradient-to-b from-black to-transparent"></div>
            {updates.reverse().map((u) => (
              <p style={{ color: 'white' }}>{u}</p>
            ))}
          </div>
        </div>
      </div>
      <AlertDialog
        open={!!error}
        title="Error Occurred"
        message={error ?? 'Unknown error occurred.'}
        actions={errorActions}
      />
    </CycledBackground>
  );
};

export default GameGenerationScreen;
