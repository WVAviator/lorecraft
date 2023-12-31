import React from 'react';
import { GameContext } from './GameProvider';

const useGameContext = () => {
  const { game, setGame } = React.useContext(GameContext);

  return { game, setGame };
};

export default useGameContext;
