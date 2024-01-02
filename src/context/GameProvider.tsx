import React from 'react';
import { Game } from '../types/Game';

interface IGameContext {
  game: Game | null;
  setGame: React.Dispatch<React.SetStateAction<Game | null>>;
}

export const GameContext = React.createContext<IGameContext>({
  game: null,
  setGame: () => {},
});

interface GameProviderProps {
  children: React.ReactNode;
}

const GameProvider: React.FC<GameProviderProps> = ({ children }) => {
  const [game, setGame] = React.useState<Game | null>(null);

  return (
    <GameContext.Provider value={{ game, setGame }}>
      {children}
    </GameContext.Provider>
  );
};

export default GameProvider;
