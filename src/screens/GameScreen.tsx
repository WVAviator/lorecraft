import InGameMenu from '../components/InGameMenu/InGameMenu';
import { BsBackpack } from 'react-icons/bs';
import SceneImage from '../components/SceneImage/SceneImage';
import useGameContext from '../hooks/useGameContext';
import { IoSettingsSharp } from 'react-icons/io5';
import { IoMdSave } from 'react-icons/io';
import { IoExitSharp } from 'react-icons/io5';
import BackgroundDiv from '../components/BackgroundDiv/BackgroundDiv';
import NarrativeWindow from '../components/NarrativeWindow/NarrativeWindow';
import React from 'react';
import useGameState from '../hooks/useGameState';
import CharacterWindow from '../components/CharacterWindow/CharacterWindow';
import PlayerEntry from '../components/PlayerEntry/PlayerEntry';
import { useNavigate } from 'react-router-dom';
import SlideoutPanel from '../components/SlideoutPanel/SlideoutPanel';
import InventoryList from '../components/InventoryList/InventoryList';
import useProcessImage from '../hooks/useProcessImage';
import SceneDescription from '../components/SceneDescription/SceneDescription';

const GameScreen = () => {
  const navigate = useNavigate();
  const { game } = useGameContext({
    redirect: '/mainmenu',
  });
  const [playerInput, setPlayerInput] = React.useState<string>('');
  const { gameState, sendNarrativeMessage, endGame } = useGameState();

  if (!gameState || !game) {
    navigate('/gamemenu');
    return null;
  }

  const currentScene = game.scenes.find(
    (scene) => scene.name === gameState.current_scene_name
  );

  return (
    <SceneImage scene={currentScene}>
      <SceneDescription scene={currentScene} />
      <NarrativeWindow
        messages={gameState.messages}
        sendMessage={(message) => sendNarrativeMessage(message)}
      />
      <InGameMenu
        menuItems={[
          {
            icon: <IoExitSharp />,
            tooltip: 'Quit Game',
            onClick: () => {
              setTimeout(() => endGame(), 0);
              navigate('/gamemenu');
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
    </SceneImage>
  );
};

export default GameScreen;
