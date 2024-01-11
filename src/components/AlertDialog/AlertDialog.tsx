import * as React from 'react';
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
  return (
    <Modal open={open} setOpen={setOpen} clickOut>
      <div className="min-h-72 min-w-96">
        <h2 className="mb-6 text-xl">{title}</h2>
        <p className="text-md">{message}</p>
        <div className="absolute bottom-2 right-2 flex items-center justify-center gap-4">
          {actions.map((action) => (
            <PromptButton
              key={action.title}
              onClick={() => action.onSelect && action.onSelect()}
            >
              {action.title}
            </PromptButton>
          ))}
        </div>
      </div>
    </Modal>
  );
};

export default AlertDialog;
