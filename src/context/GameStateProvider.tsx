import React from 'react';

interface IGameStateContext {
  gameState: GameState | null;
  setGameState: React.Dispatch<React.SetStateAction<GameState | null>>;
}

export const GameStateContext = React.createContext<IGameStateContext>({
  gameState: null,
  setGameState: () => {},
});

interface GameStateProviderProps {
  children: React.ReactNode;
}

const GameStateProvider: React.FC<GameStateProviderProps> = ({ children }) => {
  const [gameState, setGameState] = React.useState<GameState | null>(null);
  return (
    <GameStateContext.Provider value={{ gameState, setGameState }}>
      {children}
    </GameStateContext.Provider>
  );
};

export default GameStateProvider;
