import { FileEntry, readDir, removeDir } from '@tauri-apps/api/fs';
import { appLocalDataDir } from '@tauri-apps/api/path';
import React from 'react';

const useIncompleteGame = () => {
  const [incompleteGame, setIncompleteGame] = React.useState<string | null>(
    null
  );
  const [incompleteGameDirectory, setIncompleteGameDirectory] =
    React.useState<FileEntry | null>(null);

  React.useEffect(() => {
    const fetchItems = async () => {
      console.log('Checking for incomplete games...');
      try {
        const appLocalDataPath = await appLocalDataDir();
        const directories = await readDir(appLocalDataPath, {
          recursive: true,
        });

        console.log(`Searching directories at path '${directories.path}'.`);

        for (const directory of directories) {
          console.log(`Checking '${directory.name}':`);
          // Skip if it's a file or an empty directory
          if (!directory.children || directory.children.length === 0) {
            console.log(`\tIs either empty or not a directory. Skipping...`);
            continue;
          }

          // If the directory contains a game.json, then it is a complete game
          if (
            directory.children.some((file) => {
              console.log(`\t\tChecking '${file.name}' for name 'game.json'`);

              return file.name.endsWith('game.json');
            })
          ) {
            console.log(`\tContains a game.json file. Skipping...`);
            continue;
          }

          // If the directory contains a folder named tmp, then it is a game folder (not another type of folder)
          if (
            directory.children.some((directory) => {
              console.log(`\t\tChecking '${directory.name}' for name 'tmp'`);
              return directory.name.endsWith('tmp');
            })
          ) {
            console.log('\tFound incomplete game at ', directory.name);
            setIncompleteGame(directory.name);
            setIncompleteGameDirectory(directory);
            break;
          }

          console.log(
            `\tDirectory does not contain a tmp directory or a game.json file. Skipping...`
          );
        }
      } catch (error) {
        console.error('Error searching for incomplete games:', error);
      }
    };

    fetchItems();
  }, []);

  const clearIncompleteGame = async () => {
    console.log(
      `Deleting incomplete game directory at path '${incompleteGameDirectory?.path}'.`
    );
    await removeDir(incompleteGameDirectory.path, { recursive: true });
    setIncompleteGame(null);
    setIncompleteGameDirectory(null);
  };

  return { incompleteGame, clearIncompleteGame };
};

export default useIncompleteGame;
