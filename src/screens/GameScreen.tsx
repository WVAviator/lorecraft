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
import NarrativeWindow from '../components/NarrativeWindow/NarrativeWindow';
import { TextField } from '@mui/material';
import React from 'react';
import useGameState from '../hooks/useGameState';
import CharacterWindow from '../components/CharacterWindow/CharacterWindow';

const GameScreen = () => {
  const { navigateWithTransition, isTransitioning } =
    useTransitionNavigate(1000);
  const { game } = useGameContext({
    redirect: '/mainmenu',
  });
  const [playerInput, setPlayerInput] = React.useState<string>('');
  const { gameState, loading, sendNarrativeMessage } = useGameState();

  if (!gameState || !game) {
    navigateWithTransition('/gamemenu');
    return null;
  }

  return (
    <BackgroundDiv fade={isTransitioning}>
      <CharacterWindow characterInteraction={gameState.character_interaction} />
      <SplitLayout gridTemplateColumns="60% 40%">
        <SceneImage
          scene={game.scenes.find(
            (scene) => scene.id === gameState.current_scene_id
          )}
        />
        <FlexContainer
          flexDirection="column"
          padding="0.5rem"
          height="100vh"
          gap="0.5rem"
        >
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
          <NarrativeWindow messages={gameState.messages} />
          <TextField
            id="outlined-basic"
            tabIndex={0}
            variant="outlined"
            value={playerInput}
            onChange={(e) => {
              if (playerInput.length >= 497) return;
              setPlayerInput(e.target.value);
            }}
            multiline
            rows={2}
            fullWidth
            onKeyDown={(e) => {
              if (e.key === 'Enter' && !e.shiftKey) {
                e.preventDefault();
                sendNarrativeMessage(playerInput);
                setPlayerInput('');
              }
            }}
            // disabled={loading}
          />
        </FlexContainer>
      </SplitLayout>
    </BackgroundDiv>
  );
};

export default GameScreen;
