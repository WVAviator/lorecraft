import { invoke } from '@tauri-apps/api';
import React from 'react';
import { Game } from '../types/Game';
import useUpdates from '../hooks/useUpdates';
import CycledBackground from '../components/CycledBackground/CycledBackground';
import { LOADING_IMAGES } from '../data/LoadingImages';
import FlexContainer from '../components/FlexContainer/FlexContainer';
import LoadingSpinner from '../components/LoadingSpinner/LoadingSpinner';
import useGameContext from '../hooks/useGameContext';
import useTransitionNavigate from '../hooks/useTransitionNavigate';

interface GameGenerationScreenProps {}

const GameGenerationScreen: React.FC<GameGenerationScreenProps> = () => {
  const { setGame } = useGameContext();
  const { updates } = useUpdates();
  const { navigateWithTransition, isTransitioning } =
    useTransitionNavigate(1000);

  React.useEffect(() => {
    const createGame = async () => {
      const game = (await invoke('create_new_game', { prompt: '' })) as Game;
      setGame(game);
      navigateWithTransition('/gamemenu');
    };
    createGame();
  }, []);

  return (
    <CycledBackground images={LOADING_IMAGES} play={!isTransitioning}>
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
