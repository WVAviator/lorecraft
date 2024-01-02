import * as React from 'react';
import Button from '@mui/material/Button';
import Dialog from '@mui/material/Dialog';
import DialogActions from '@mui/material/DialogActions';
import DialogContent from '@mui/material/DialogContent';
import DialogContentText from '@mui/material/DialogContentText';
import DialogTitle from '@mui/material/DialogTitle';

export interface Action {
  title: string;
  onSelect?: () => void;
}

interface AlertDialogProps {
  open: boolean;
  setOpen?: (open: boolean) => void;
  title: string;
  message: string;
  actions: Action[];
}

const AlertDialog: React.FC<AlertDialogProps> = ({
  open,
  setOpen,
  title,
  message,
  actions,
}) => {
  const handleClose = () => {
    setOpen && setOpen(false);
  };

  return (
    <Dialog
      open={open}
      onClose={handleClose}
      aria-labelledby="alert-dialog-title"
      aria-describedby="alert-dialog-description"
    >
      <DialogTitle id="alert-dialog-title">{title}</DialogTitle>
      <DialogContent>
        <DialogContentText id="alert-dialog-description">
          {message}
        </DialogContentText>
      </DialogContent>
      <DialogActions>
        {actions.map((action) => (
          <Button onClick={() => action.onSelect && action.onSelect()}>
            {action.title}
          </Button>
        ))}
      </DialogActions>
    </Dialog>
  );
};

export default AlertDialog;
