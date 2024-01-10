import * as React from 'react';
import Button from '@mui/material/Button';
import Dialog from '@mui/material/Dialog';
import DialogActions from '@mui/material/DialogActions';
import DialogContent from '@mui/material/DialogContent';
import DialogContentText from '@mui/material/DialogContentText';
import DialogTitle from '@mui/material/DialogTitle';
import Modal from '../Modal/Modal';
import PromptButton from '../PromptButton/PromptButton';

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
    <Modal open={open} setOpen={setOpen}>
      <h2 className="mb-6 text-xl">{title}</h2>
      <p className="text-md">{message}</p>
      <div className="absolute bottom-4 right-4 flex items-center justify-center gap-4">
        {actions.map((action) => (
          <PromptButton key={action.title}>{action.title}</PromptButton>
        ))}
      </div>
    </Modal>
  );
};

export default AlertDialog;
