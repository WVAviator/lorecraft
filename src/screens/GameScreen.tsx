import SceneImage from '../components/SceneImage/SceneImage';
import SplitLayout from '../components/SplitLayout/SplitLayout';
import useGameContext from '../hooks/useGameContext';

const GameScreen = () => {
  const { game } = useGameContext({
    redirect: '/mainmenu',
  });
  return (
    <div>
      <SplitLayout gridTemplateColumns="60% 40%">
        <SceneImage scene={game?.scenes[0]} />
        <div>Other stuff</div>
      </SplitLayout>
    </div>
  );
};

export default GameScreen;
