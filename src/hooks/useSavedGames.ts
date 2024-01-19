import { readDir, readTextFile } from '@tauri-apps/api/fs';
import { appLocalDataDir } from '@tauri-apps/api/path';
import React from 'react';
import { Game } from '../types/Game';

const useSavedGames = () => {
  const [games, setGames] = React.useState<Game[]>([]);
  React.useEffect(() => {
    const fetchItems = async () => {
      try {
        const appLocalDataPath = await appLocalDataDir();
        const directories = await readDir(appLocalDataPath, {
          recursive: true,
        });

        for (const directory of directories) {
          if (directory.children) {
            for (const file of directory.children) {
              if (file.name?.endsWith('game.json')) {
                const fileContent = await readTextFile(file.path);
                const game = JSON.parse(fileContent) as Game;

                console.log('Game Narrative Page 0: ', game.narrative.pages[0]);
                setGames((prevGames) => {
                  return prevGames.includes(game)
                    ? prevGames
                    : [...prevGames, game];
                });
              }
            }
          }
        }
      } catch (error) {
        console.error('Error fetching items:', error);
      }
    };

    fetchItems();
  }, []);

  return { games };
};

export default useSavedGames;
