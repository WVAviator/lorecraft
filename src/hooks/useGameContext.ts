import React from 'react';
import { GameContext } from '../context/GameProvider';
import { useNavigate } from 'react-router-dom';

interface UseGameContextOptions {
  /**
   * Redirect to this page if 'game' is null
   */
  redirect?: string;
}

const useGameContext = ({ redirect }: Partial<UseGameContextOptions> = {}) => {
  const { game, setGame } = React.useContext(GameContext);
  const navigate = useNavigate();

  React.useEffect(() => {
    if (redirect && !game) {
      navigate(redirect);
    }
  }, [redirect]);

  return { game, setGame };
};

export default useGameContext;
