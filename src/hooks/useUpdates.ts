import { UnlistenFn } from '@tauri-apps/api/event';
import { appWindow } from '@tauri-apps/api/window';
import React from 'react';

const useUpdates = () => {
  const [updates, setUpdates] = React.useState<string[]>([]);

  React.useEffect(() => {
    let unlisten: UnlistenFn;
    const getUpdates = async () => {
      unlisten = await appWindow.listen('updates', (e) => {
        setUpdates((updates) => [...updates, e.payload as string]);
      });
    };
    getUpdates();

    return () => {
      unlisten && unlisten();
    };
  }, []);

  return { updates };
};

export default useUpdates;
