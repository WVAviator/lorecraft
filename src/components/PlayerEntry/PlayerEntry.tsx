import { TextField } from '@mui/material';
import React from 'react';

interface PlayerEntryProps {
  value: string;
  onChange: (event: React.ChangeEvent<HTMLInputElement>) => void;
  onSubmit?: () => void;
  disabled?: boolean;
}

const PlayerEntry: React.FC<PlayerEntryProps> = ({
  onChange,
  value,
  onSubmit = () => {},
  disabled = false,
}) => {
  const inputRef = React.useRef<HTMLInputElement>();

  React.useEffect(() => {
    inputRef.current?.focus();
  }, []);

  return (
    <TextField
      id="outlined-basic"
      inputRef={inputRef}
      variant="outlined"
      value={value}
      onChange={onChange}
      multiline
      rows={2}
      fullWidth
      onKeyDown={(e) => {
        if (e.key === 'Enter' && !e.shiftKey) {
          e.preventDefault();
          onSubmit();
        }
      }}
      disabled={disabled}
    />
  );
};

export default PlayerEntry;
