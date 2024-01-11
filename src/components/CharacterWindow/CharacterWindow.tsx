import useProcessImage from '../../hooks/useProcessImage';
import useGameContext from '../../hooks/useGameContext';
import CharacterConversation from '../CharacterConversation/CharacterConversation';
import React from 'react';
import useGameState from '../../hooks/useGameState';
import PlayerEntry from '../PlayerEntry/PlayerEntry';
import Modal from '../Modal/Modal';
import AlertDialog from '../AlertDialog/AlertDialog';

interface CharacterWindowProps {
  characterInteraction: CharacterInteraction | null;
}

const CharacterWindow: React.FC<CharacterWindowProps> = ({
  characterInteraction,
}) => {
  const { game } = useGameContext();
  const [playerInput, setPlayerInput] = React.useState('');
  const {
    sendCharacterMessage,
    characterTradeResponse,
    endCharacterConversation,
    loading,
  } = useGameState();

  if (!game || !characterInteraction) {
    return null;
  }

  const character = game.characters.find(
    (ch) => ch.id === characterInteraction.character_id
  );

  if (!character) return null;

  const { src, alt } = useProcessImage(character.image);

  return (
    <>
      <Modal
        open={true}
        setOpen={(open: boolean) => {
          if (!open) {
            endCharacterConversation();
          }
        }}
        clickOut
      >
        <div className="grid w-[70vw] grid-cols-2 gap-4">
          <div>
            <h3 className="font-overlock-sc text-lg">{character.name}</h3>
            <img src={src} alt={alt} className="object-cover shadow-inner" />
          </div>
          <div className="flex flex-col gap-2">
            <div className="flex h-full flex-grow">
              <CharacterConversation messages={characterInteraction.messages} />
            </div>
            <div>
              <PlayerEntry
                value={playerInput}
                onChange={(e) => {
                  if (playerInput.length >= 120) return;
                  setPlayerInput(e.target.value);
                }}
                onSubmit={() => {
                  sendCharacterMessage(playerInput);
                  setPlayerInput('');
                }}
                placeholder="What do you want to say?"
                // disabled={loading}
              />
            </div>
          </div>
        </div>
      </Modal>

      <AlertDialog
        open={!!characterInteraction.trade}
        setOpen={(open: boolean) => {
          if (!open) {
            characterTradeResponse(false);
          }
        }}
        title="Trade Request"
        message={
          characterInteraction.trade?.from_player
            ? `${character.name} wishes to trade their ${characterInteraction.trade?.to_player} for your ${characterInteraction.trade?.from_player}. Do you accept?`
            : `${character.name} wishes to give you their ${characterInteraction.trade?.to_player}. Do you accept?`
        }
        actions={[
          {
            title: 'Accept',
            onSelect: () => characterTradeResponse(true),
          },
          {
            title: 'Decline',
            onSelect: () => characterTradeResponse(false),
          },
        ]}
      />
    </>
  );
};

export default CharacterWindow;
