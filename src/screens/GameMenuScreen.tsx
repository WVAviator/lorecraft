import AbsoluteContainer from '../components/AbsoluteContainer/AbsoluteContainer';
import BackgroundDiv from '../components/BackgroundDiv/BackgroundDiv';
import GameMenu from '../components/GameMenu/GameMenu';
import GameMenuListItem from '../components/GameMenuListItem/GameMenuListItem';
import GameTitle from '../components/GameTitle/GameTitle';
import MenuList from '../components/MenuList/MenuList';
import MenuListItem from '../components/MenuListItem/MenuListItem';
import useGameContext from '../hooks/useGameContext';
import useProcessImage from '../hooks/useProcessImage';
import useTransitionNavigate from '../hooks/useTransitionNavigate';

const GameMenuScreen = () => {
  const { game, setGame } = useGameContext();

  const { navigateWithTransition, isTransitioning } =
    useTransitionNavigate(1000);

  const { src, alt } = useProcessImage(game?.cover_art);

  return (
    <BackgroundDiv image={src} alt={alt} fade={isTransitioning}>
      <AbsoluteContainer top="3rem" left="3rem">
        <GameTitle>{game?.name || ''}</GameTitle>
      </AbsoluteContainer>
      <GameMenu>
        <GameMenuListItem
          onClick={() => {
            navigateWithTransition('/narrative');
          }}
        >
          New Game
        </GameMenuListItem>
        <GameMenuListItem>Continue</GameMenuListItem>
        <GameMenuListItem
          onClick={() => {
            navigateWithTransition('/mainmenu', () => {
              setGame(null);
            });
          }}
        >
          Back to Main Menu
        </GameMenuListItem>
      </GameMenu>
    </BackgroundDiv>
  );
};

export default GameMenuScreen;
