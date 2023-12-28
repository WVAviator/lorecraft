import { invoke } from '@tauri-apps/api';
import React from 'react';
import { Game } from '../types/Game';
import BackgroundDiv from '../components/BackgroundDiv/BackgroundDiv';
import AbsoluteContainer from '../components/AbsoluteContainer/AbsoluteContainer';
import useUpdates from '../hooks/useUpdates';
import CycledBackground from '../components/CycledBackground/CycledBackground';
import { LOADING_IMAGES } from '../data/LoadingImages';
import FlexContainer from '../components/FlexContainer/FlexContainer';
import LoadingSpinner from '../components/LoadingSpinner/LoadingSpinner';
import { convertFileSrc } from '@tauri-apps/api/tauri';

interface GameGenerationScreenProps {}

const GameGenerationScreen: React.FC<GameGenerationScreenProps> = () => {
  const [game, setGame] = React.useState<Game | null>(null);
  const { updates } = useUpdates();

  React.useEffect(() => {
    const createGame = async () => {
      console.log('Sending request to backend to form new game.');
      const game = (await invoke('create_new_game', { prompt: '' })) as Game;
      console.log(`Received game from backend.\n${game}`);
      game.cover_art.src = convertFileSrc(game.cover_art.src);
      setGame(game);
    };
    createGame();
    return () => {};
  }, []);

  if (!game) {
    return (
      <CycledBackground images={LOADING_IMAGES}>
        <FlexContainer
          alignItems="flex-end"
          width="100%"
          height="100%"
          padding="0.5rem"
        >
          <FlexContainer height="5rem" gap="1rem">
            <LoadingSpinner />
            <p style={{ backgroundColor: 'black', color: 'white' }}>
              {updates.length > 0 ? updates[updates.length - 1] : ''}
            </p>
          </FlexContainer>
        </FlexContainer>
      </CycledBackground>
    );
  }

  return (
    <BackgroundDiv
      image={game?.cover_art.src || ''}
      alt={game?.cover_art.alt || ''}
    >
      <AbsoluteContainer top="75%" height="10rem" backgroundColor="black">
        <h1 style={{ color: 'white' }}>{game?.name || 'Loading...'}</h1>
      </AbsoluteContainer>
    </BackgroundDiv>
  );
};

export default GameGenerationScreen;
