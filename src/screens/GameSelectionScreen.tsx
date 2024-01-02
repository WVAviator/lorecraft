import BackgroundDiv from '../components/BackgroundDiv/BackgroundDiv';
import ContainerCarousel from '../components/ContainerCarousel/ContainerCarousel';
import GameSummaryCard from '../components/GameSummaryCard/GameSummaryCard';
import useGameContext from '../hooks/useGameContext';
import useSavedGames from '../hooks/useSavedGames';
import useTransitionNavigate from '../hooks/useTransitionNavigate';
import { Game } from '../types/Game';
import background from '/images/menu/stone_hall.png';

const BACKGROUND_ALT_DESC =
  'a stone wall with intricate arcane patterns carved into it';

const GameSelectionScreen = () => {
  const { games } = useSavedGames();
  const { setGame } = useGameContext();
  const { navigateWithTransition, isTransitioning } =
    useTransitionNavigate(1000);

  const handleClick = (game: Game) => {
    setGame(game);
    navigateWithTransition('/gamemenu');
  };

  return (
    <BackgroundDiv
      image={background}
      alt={BACKGROUND_ALT_DESC}
      fade={isTransitioning}
    >
      <ContainerCarousel
        inactiveItemProps={{ faceDown: true }}
        activeItemProps={{ faceDown: false }}
        sizeDiff={0.3}
        spacing={0.4}
      >
        {games.map((game) => {
          return (
            <GameSummaryCard
              key={game.id}
              game={game}
              onClick={() => handleClick(game)}
            />
          );
        })}
      </ContainerCarousel>
    </BackgroundDiv>
  );
};

export default GameSelectionScreen;
