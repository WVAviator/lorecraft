import { convertFileSrc } from '@tauri-apps/api/tauri';
import { readDir, readTextFile } from '@tauri-apps/api/fs';
import { appLocalDataDir, join } from '@tauri-apps/api/path';
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

        console.log('directories', directories);

        for (const directory of directories) {
          if (!directory.name) {
            continue;
          }

          if (directory.children) {
            for (const file of directory.children) {
              console.log('file', file);
              if (file.name?.endsWith('game.json')) {
                console.log('reading game filePath', file.path);
                const fileContent = await readTextFile(file.path);
                const game = JSON.parse(fileContent) as Game;
                setGames((prevGames) => [...prevGames, game]);
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
