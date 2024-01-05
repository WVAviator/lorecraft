import { useNavigate } from 'react-router-dom';
import useGameContext from '../hooks/useGameContext';
import React from 'react';
import BackgroundDiv from '../components/BackgroundDiv/BackgroundDiv';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import AbsoluteContainer from '../components/AbsoluteContainer/AbsoluteContainer';
import useGameState from '../hooks/useGameState';

const NarrativeScreen = () => {
  const { game } = useGameContext();
  const navigate = useNavigate();

  const { startGame } = useGameState();

  React.useEffect(() => {
    if (!game) {
      navigate('/mainmenu');
      return;
    }
    startGame(game.id);
  }, [game, startGame]);

  const [pageIndex, setPageIndex] = React.useState(0);
  const [fade, setFade] = React.useState(false);

  React.useEffect(() => {
    let timeout = setTimeout(() => {
      handleClick();
    }, 8000);

    return () => {
      clearTimeout(timeout);
    };
  }, [pageIndex]);

  const handleClick = () => {
    setFade(true);
    setTimeout(() => {
      if (pageIndex >= (game?.narrative.pages.length ?? 0) - 1) {
        navigate('/game');
        return;
      }
      setPageIndex((pageIndex) => pageIndex + 1);
      setFade(false);
    }, 1000);
  };

  let currentPage = game?.narrative.pages[pageIndex];
  if (!currentPage) return null;
  let src = convertFileSrc(currentPage.image.src);
  let alt = currentPage.image.alt;
  let { narrative } = currentPage;

  return (
    <BackgroundDiv image={src} alt={alt} onClick={handleClick} fade={fade}>
      <AbsoluteContainer bottom="0" left="0" right="0">
        <p
          style={{ backgroundColor: 'black', color: 'white', padding: '1rem' }}
        >
          {narrative}
        </p>
      </AbsoluteContainer>
    </BackgroundDiv>
  );
};

export default NarrativeScreen;
