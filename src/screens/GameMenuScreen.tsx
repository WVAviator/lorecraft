import { useNavigate } from 'react-router-dom';
import BackgroundDiv from '../components/BackgroundDiv/BackgroundDiv';
import GameMenu from '../components/GameMenu/GameMenu';
import GameMenuListItem from '../components/GameMenuListItem/GameMenuListItem';
import GameTitle from '../components/GameTitle/GameTitle';
import useGameContext from '../hooks/useGameContext';
import useProcessImage from '../hooks/useProcessImage';

const GameMenuScreen = () => {
  const { game, setGame } = useGameContext();

  const navigate = useNavigate();

  const { src, alt } = useProcessImage(game?.cover_art);

  return (
    <BackgroundDiv image={src} alt={alt} fade={false}>
      <audio src={game?.title_music.src} loop autoPlay />
      <div className="absolute left-12 top-12">
        <GameTitle>{game?.name || ''}</GameTitle>
      </div>
      <GameMenu>
        <GameMenuListItem
          onClick={() => {
            navigate('/narrative');
          }}
        >
          New Game
        </GameMenuListItem>
        <GameMenuListItem>Continue</GameMenuListItem>
        <GameMenuListItem
          onClick={() => {
            setTimeout(() => setGame(null), 500);
            navigate('/mainmenu');
          }}
        >
          Back to Main Menu
        </GameMenuListItem>
      </GameMenu>
    </BackgroundDiv>
  );
};

export default GameMenuScreen;
