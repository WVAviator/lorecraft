import {
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogContentText,
  DialogTitle,
  LinearProgress,
  Modal,
  TextField,
} from '@mui/material';
import useProcessImage from '../../hooks/useProcessImage';
import useGameContext from '../../hooks/useGameContext';
import styles from './CharacterWindow.module.css';
import NarrativeWindow from '../NarrativeWindow/NarrativeWindow';
import CharacterConversation from '../CharacterConversation/CharacterConversation';
import React from 'react';
import useGameState from '../../hooks/useGameState';

interface CharacterWindowProps {
  characterInteraction: CharacterInteraction | null;
}

const CharacterWindow: React.FC<CharacterWindowProps> = ({
  characterInteraction,
}) => {
  const { game } = useGameContext();
  const [playerInput, setPlayerInput] = React.useState('');
  const { sendCharacterMessage, characterTradeResponse, loading } =
    useGameState();

  if (!game || !characterInteraction) {
    return null;
  }

  const character = game.characters.find(
    (ch) => ch.id === characterInteraction.character_id
  );

  if (!character) return null;

  const { src, alt } = useProcessImage(character.image);

  return (
    <Modal open={true}>
      <>
        <Dialog open={!!characterInteraction.trade}>
          <DialogTitle>Trade Request</DialogTitle>
          <DialogContent>
            <DialogContentText>
              {characterInteraction.trade?.from_player
                ? `${character.name} wishes to trade their ${characterInteraction.trade?.to_player} for your ${characterInteraction.trade?.from_player}. Do you accept?`
                : `${character.name} wishes to give you their ${characterInteraction.trade?.to_player}. Do you accept?`}
            </DialogContentText>
          </DialogContent>

          <DialogActions>
            <Button
              disabled={loading}
              onClick={() => characterTradeResponse(true)}
            >
              Accept
            </Button>
            <Button
              disabled={loading}
              onClick={() => characterTradeResponse(false)}
            >
              Decline
            </Button>
          </DialogActions>
          {loading && <LinearProgress />}
        </Dialog>
        <div className={styles.window}>
          <img src={src} alt={alt} />
          <div className={styles.messages}>
            <CharacterConversation messages={characterInteraction.messages} />
            <TextField
              id="outlined-basic"
              tabIndex={0}
              variant="outlined"
              value={playerInput}
              onChange={(e) => {
                if (playerInput.length >= 120) return;
                setPlayerInput(e.target.value);
              }}
              multiline
              rows={2}
              fullWidth
              onKeyDown={(e) => {
                if (e.key === 'Enter' && !e.shiftKey) {
                  e.preventDefault();
                  sendCharacterMessage(playerInput);
                  setPlayerInput('');
                }
              }}
              disabled={loading}
            />
          </div>
        </div>
      </>
    </Modal>
  );
};

export default CharacterWindow;
