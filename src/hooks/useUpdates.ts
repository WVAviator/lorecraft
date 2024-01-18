import { UnlistenFn } from '@tauri-apps/api/event';
import { appWindow } from '@tauri-apps/api/window';
import React from 'react';

const useUpdates = () => {
  const [updates, setUpdates] = React.useState<string[]>([]);
  const [gameId, setGameId] = React.useState<string | null>(null);

  React.useEffect(() => {
    let unlisten: UnlistenFn;
    const getUpdates = async () => {
      unlisten = await appWindow.listen<GameGenerationUpdate>(
        'updates',
        (e) => {
          setUpdates((updates) => [...updates, e.payload.message]);
          setGameId(e.payload.game_id);
        }
      );
    };
    getUpdates();

    return () => {
      unlisten && unlisten();
    };
  }, []);

  return { updates, gameId };
};

export default useUpdates;
