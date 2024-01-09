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
      <h2 className="text-xl mb-6">{title}</h2>
      <p className="text-md">{message}</p>
      <div className="absolute bottom-4 right-4 flex justify-center items-center gap-4">
        {actions.map((action) => (
          <PromptButton>{action.title}</PromptButton>
        ))}
      </div>
    </Modal>
  );
};
//   {/* <Dialog */ }
//   {/*   open={open} */ }
// {/*   onClose={handleClose} */ }
// {/*   aria-labelledby="alert-dialog-title" */ }
// {/*   aria-describedby="alert-dialog-description" */ }
// {/* > */ }
// {/*   <DialogTitle id="alert-dialog-title">{title}</DialogTitle> */ }
// {/*   <DialogContent> */ }
// {/*     <DialogContentText id="alert-dialog-description"> */ }
// {/*       {message} */ }
// {/*     </DialogContentText> */ }
// {/*   </DialogContent> */ }
// {/*   <DialogActions> */ }
// {/*     {actions.map((action) => ( */ }
// {/*       <Button onClick={() => action.onSelect && action.onSelect()}> */ }
// {/*         {action.title} */ }
// {/*       </Button> */ }
// {/*     ))} */ }
// {/*   </DialogActions> */ }
// {/* </Dialog> */ }

export default AlertDialog;
