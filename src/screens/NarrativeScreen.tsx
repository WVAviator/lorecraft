import { useNavigate } from 'react-router-dom';
import useGameContext from '../hooks/useGameContext';
import React from 'react';
import BackgroundDiv from '../components/BackgroundDiv/BackgroundDiv';
import { convertFileSrc } from '@tauri-apps/api/tauri';
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

  const audioRef = React.useRef<HTMLAudioElement>(null);

  React.useEffect(() => {
    if (!audioRef.current) return;
    audioRef.current.play();

    let timeout: NodeJS.Timeout;

    const handleEnded = () => {
      timeout = setTimeout(handleClick, 1000);
    };

    audioRef.current.addEventListener('ended', handleEnded);

    return () => {
      clearTimeout(timeout);
      audioRef.current?.removeEventListener('ended', handleEnded);
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
  console.log(`Narrative page ${pageIndex} loaded: `);
  console.debug(currentPage);
  if (!currentPage) return null;
  let src = convertFileSrc(currentPage.image.src);
  let alt = currentPage.image.alt;
  let audioSrc = convertFileSrc(currentPage.audio?.src ?? '');
  console.log('Converted audio src: ', audioSrc);
  let { narrative } = currentPage;

  return (
    <BackgroundDiv
      image={src}
      alt={alt}
      onClick={handleClick}
      fade={fade}
      randomPan
    >
      <audio ref={audioRef} src={audioSrc} autoPlay />
      <div className="absolute bottom-[10%] left-[10%] right-0">
        <div className="rounded-l-md bg-black bg-opacity-80 px-4 py-6">
          <p>{narrative}</p>
        </div>
      </div>
    </BackgroundDiv>
  );
};

export default NarrativeScreen;
