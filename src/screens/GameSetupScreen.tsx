import { invoke } from '@tauri-apps/api';
import { UnlistenFn } from '@tauri-apps/api/event';
import { appWindow } from '@tauri-apps/api/window';
import React from 'react';
import { Game } from '../types/Game';

interface GameSetupScreenProps {}

const GameSetupScreen: React.FC<GameSetupScreenProps> = () => {
  const [game, setGame] = React.useState<Game | null>(null);
  React.useEffect(() => {
    let unlisten: UnlistenFn;
    const createGame = async () => {
      console.log('Sending request to backend to form new game.');
      const id = await invoke('create_new_game', { prompt: '' });
      console.log(
        `Received response from backend with game id ${id}. Subscribing to game creation event.`
      );
      unlisten = await appWindow.listen(`create:${id}`, (event) => {
        console.log(
          `Received game creation event with payload ${JSON.stringify(
            event.payload
          )}.`
        );
        const game = event.payload as Game;
        setGame(game);
      });
    };
    createGame();
    return () => {
      unlisten && unlisten();
    };
  });
  return (
    <div>
      {game ? <div>{JSON.stringify(game)}</div> : <div>Loading...</div>}
    </div>
  );
};

export default GameSetupScreen;
