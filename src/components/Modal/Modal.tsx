import IntricateFrame from '../IntricateFrame/IntricateFrame';

interface ModalProps {
  open: boolean;
  setOpen?: (open: boolean) => void;
  children: React.ReactNode;
  clickOut?: boolean;
}

const Modal: React.FC<ModalProps> = ({
  open,
  children,
  setOpen = () => {},
  clickOut = true,
}) => {
  return (
    <div
      className={`absolute bottom-0 left-0 right-0 top-0 bg-gray-300 bg-opacity-25 backdrop-blur-sm ${
        open ? 'block' : 'hidden'
      }`}
      onClick={() => {
        if (clickOut && setOpen) {
          setOpen(false);
        }
      }}
    >
      <div className="relative h-full w-full">
        <div className="absolute left-[50%] top-[50%] min-h-80 min-w-96 translate-x-[-50%] translate-y-[-50%] rounded-md bg-blue-950 p-1 shadow-xl">
          <IntricateFrame>{children}</IntricateFrame>
        </div>
      </div>
    </div>
  );
};

export default Modal;
