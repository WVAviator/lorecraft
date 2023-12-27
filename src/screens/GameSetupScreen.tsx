import { invoke } from '@tauri-apps/api';
import { UnlistenFn } from '@tauri-apps/api/event';
import { appWindow } from '@tauri-apps/api/window';
import React from 'react';
import { Game } from '../types/Game';

interface GameSetupScreenProps {}

const GameSetupScreen: React.FC<GameSetupScreenProps> = () => {
  const [game, setGame] = React.useState<Game | null>(null);
  React.useEffect(() => {
    const createGame = async () => {
      console.log('Sending request to backend to form new game.');
      const game = (await invoke('create_new_game', { prompt: '' })) as Game;
      setGame(game);
    };
    createGame();
  }, []);

  return (
    <div>
      {game ? <div>{JSON.stringify(game)}</div> : <div>Loading...</div>}
    </div>
  );
};

export default GameSetupScreen;
