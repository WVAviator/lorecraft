import FlexContainer from '../components/FlexContainer/FlexContainer';
import InGameMenu from '../components/InGameMenu/InGameMenu';
import SceneImage from '../components/SceneImage/SceneImage';
import SplitLayout from '../components/SplitLayout/SplitLayout';
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
import TextArea from '../components/TextArea/TextArea';

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

  return (
    <BackgroundDiv fade={false}>
      <CharacterWindow
        characterInteraction={
          gameState.character_interaction?.closed
            ? null
            : gameState.character_interaction
        }
      />
      <div className="grid grid-cols-[60%_40%] bg-blue-950">
        <SceneImage
          scene={game.scenes.find(
            (scene) => scene.id === gameState.current_scene_id
          )}
        />
        <div className="flex h-full flex-col gap-2 p-2">
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
          <NarrativeWindow messages={gameState.messages} />
          <PlayerEntry
            value={playerInput}
            onChange={(e) => {
              if (playerInput.length >= 497) return;
              setPlayerInput(e.target.value);
            }}
            onSubmit={() => {
              sendNarrativeMessage(playerInput);
              setPlayerInput('');
            }}
            placeholder="What do you want to do?"
            rows={2}
            // disabled={loading}
          />
        </div>
      </div>
    </BackgroundDiv>
  );
};

export default GameScreen;
