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

interface GameGenerationScreenProps {}

const GameGenerationScreen: React.FC<GameGenerationScreenProps> = () => {
  const { setGame } = useGameContext();
  const { updates } = useUpdates();
  const navigate = useNavigate();
  const location = useLocation();

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
      const response = (await invoke('create_new_game', {
        request: location.state.request,
      })) as CreateNewGameResponse;
      if (response.success === false || !response.game) {
        console.error('Failed to generate game.');
        navigate('/mainmenu');
        return;
      }
      setGame(response.game);
      navigate('/gamemenu');
    };
    createGame();
  }, []);

  return (
    <CycledBackground images={LOADING_IMAGES} play={!false}>
      <div className="flex h-full w-full items-end p-2">
        <div className="flex h-20 gap-4 bg-black">
          <LoadingSpinner />
          <p style={{ color: 'white' }}>
            {updates.length > 0 ? updates[updates.length - 1] : ''}
          </p>
        </div>
      </div>
    </CycledBackground>
  );
};

export default GameGenerationScreen;
