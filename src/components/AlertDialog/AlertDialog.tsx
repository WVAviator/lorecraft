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
    <div className={`absolute top-0 bottom-0 left-0 right-0 bg-gray-300 backdrop-blur-sm bg-opacity-25 ${open ? "block" : "hidden"}`}>
      <div className="relative w-full h-full">
        <div className="absolute top-[50%] left-[50%] translate-x-[-50%] translate-y-[-50%] min-w-96 min-h-80 bg-blue-950 rounded-md shadow-xl p-6">
          <h2 className="text-lg">{title}</h2>
          <p>{message}</p>
          <div className="absolute bottom-0 right-0 mr-2 mb-2">
            {actions.map((action) =>
              (<button className="text-md bg-transparent p-1 rounded-md hover:bg-blue-800">{action.title}</button>)
            )}
          </div>
        </div>
      </div>
    </div>
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
