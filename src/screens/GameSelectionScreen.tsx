import BackgroundDiv from '../components/BackgroundDiv/BackgroundDiv';
import GameSummaryCard from '../components/GameSummaryCard/GameSummaryCard';
import useSavedGames from '../hooks/useSavedGames';
import background from '/images/menu/stone_hall.png';

const BACKGROUND_ALT_DESC =
  'a stone wall with intricate arcane patterns carved into it';

const GameSelectionScreen = () => {
  const { games } = useSavedGames();
  return (
    <BackgroundDiv image={background} alt={BACKGROUND_ALT_DESC}>
      {games.map((game) => {
        return <GameSummaryCard key={game.id} game={game} />;
      })}
    </BackgroundDiv>
  );
};

export default GameSelectionScreen;
