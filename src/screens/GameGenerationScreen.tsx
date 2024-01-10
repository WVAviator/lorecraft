import { invoke } from '@tauri-apps/api';
import React from 'react';
import { Game } from '../types/Game';
import useUpdates from '../hooks/useUpdates';
import CycledBackground from '../components/CycledBackground/CycledBackground';
import { LOADING_IMAGES } from '../data/LoadingImages';
import FlexContainer from '../components/FlexContainer/FlexContainer';
import LoadingSpinner from '../components/LoadingSpinner/LoadingSpinner';
import useGameContext from '../hooks/useGameContext';
import {
  CreateNewGameRequest,
  CreateNewGameResponse,
} from '../types/CreateNewGame';
import { useNavigate } from 'react-router-dom';

interface GameGenerationScreenProps {}

const GameGenerationScreen: React.FC<GameGenerationScreenProps> = () => {
  const { setGame } = useGameContext();
  const { updates } = useUpdates();
  const navigate = useNavigate();

  React.useEffect(() => {
    const createGame = async () => {
      const request: CreateNewGameRequest = {
        prompt: '',
      };
      const response = (await invoke('create_new_game', {
        request,
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
      <FlexContainer
        alignItems="flex-end"
        width="100%"
        height="100%"
        padding="0.5rem"
      >
        <FlexContainer height="5rem" gap="1rem" backgroundColor="black">
          <LoadingSpinner />
          <p style={{ color: 'white' }}>
            {updates.length > 0 ? updates[updates.length - 1] : ''}
          </p>
        </FlexContainer>
      </FlexContainer>
    </CycledBackground>
  );
};

export default GameGenerationScreen;
