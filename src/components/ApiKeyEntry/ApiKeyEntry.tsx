import {
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogContentText,
  DialogTitle,
  TextField,
} from '@mui/material';
import React from 'react';

interface ApiKeyEntryProps {
  open: boolean;
  setOpen: (open: boolean) => void;
  onSubmit: (value: string) => void;
}

const ApiKeyEntry: React.FC<ApiKeyEntryProps> = ({
  open,
  setOpen,
  onSubmit,
}) => {
  const [value, setValue] = React.useState('');
  const handleSubmit = () => {
    if (!value) return;
    setOpen(false);
    onSubmit(value);
    setValue('');
  };
  const handleHelp = () => {
    console.log('Help');
  };
  const handleClose = () => {
    window.close();
  };
  return (
    <Dialog open={open} onClose={handleClose}>
      <DialogTitle>API Key</DialogTitle>
      <DialogContent>
        <DialogContentText mb={2}>
          Lorecraft uses OpenAI to generate games and gameplay. Please provide
          your OpenAI API key to play.
        </DialogContentText>

        <TextField
          autoFocus
          margin="dense"
          id="apikey"
          label="OpenAI API Key"
          fullWidth
          variant="outlined"
          value={value}
          onChange={(e) => setValue(e.target.value)}
          focused
        />
      </DialogContent>
      <DialogActions>
        <Button onClick={handleHelp}>Help</Button>
        <Button onClick={handleSubmit} disabled={!value}>
          Submit
        </Button>
      </DialogActions>
    </Dialog>
  );
};

export default ApiKeyEntry;
