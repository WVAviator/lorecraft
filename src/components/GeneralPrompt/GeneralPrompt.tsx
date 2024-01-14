import Modal from '../Modal/Modal';
import PromptButton from '../PromptButton/PromptButton';

interface GeneralPromptProps {
  open: boolean;
  setOpen: (open: boolean) => void;
  title: string;
  description: string;
  onSubmit: () => void;
  children: React.ReactNode;
}

const GeneralPrompt: React.FC<GeneralPromptProps> = ({
  open,
  setOpen,
  title,
  description,
  onSubmit,
  children,
}) => {
  return (
    <Modal open={open} setOpen={setOpen} className="pb-24">
      <h2 className="mb-6 text-xl">{title}</h2>
      <p className="text-md mb-6">{description}</p>
      {children}
      <div className="absolute bottom-4 right-4 flex items-center justify-center gap-4">
        <PromptButton onClick={() => onSubmit()}>Submit</PromptButton>
      </div>
      <div className="h-24"></div>
    </Modal>
  );
};
export default GeneralPrompt;
