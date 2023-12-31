import FlexContainer from '../components/FlexContainer/FlexContainer';
import InGameMenu from '../components/InGameMenu/InGameMenu';
import SceneImage from '../components/SceneImage/SceneImage';
import SplitLayout from '../components/SplitLayout/SplitLayout';
import useGameContext from '../hooks/useGameContext';
import { IoSettingsSharp } from 'react-icons/io5';
import { IoMdSave } from 'react-icons/io';
import { IoExitSharp } from 'react-icons/io5';
import useTransitionNavigate from '../hooks/useTransitionNavigate';
import BackgroundDiv from '../components/BackgroundDiv/BackgroundDiv';

const GameScreen = () => {
  const { navigateWithTransition, isTransitioning } =
    useTransitionNavigate(1000);
  const { game } = useGameContext({
    redirect: '/mainmenu',
  });
  return (
    <BackgroundDiv fade={isTransitioning}>
      <SplitLayout gridTemplateColumns="60% 40%">
        <SceneImage scene={game?.scenes[0]} />
        <FlexContainer flexDirection="column" padding="0.5rem">
          <InGameMenu
            menuItems={[
              {
                icon: <IoExitSharp />,
                tooltip: 'Quit Game',
                onClick: () => {
                  navigateWithTransition('/gamemenu');
                },
              },
              {
                icon: <IoMdSave />,
                tooltip: 'Save Game',
                onClick: () => {
                  console.log('Saved game!');
                },
              },
              {
                icon: <IoSettingsSharp />,
                tooltip: 'Settings',
                onClick: () => {
                  console.log('Settings clicked!');
                },
              },
            ]}
          />
        </FlexContainer>
      </SplitLayout>
    </BackgroundDiv>
  );
};

export default GameScreen;
