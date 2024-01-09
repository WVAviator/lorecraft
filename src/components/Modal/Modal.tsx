import IntricateFrame from '../intricateFrame/IntricateFrame';

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
      className={`absolute top-0 bottom-0 left-0 right-0 bg-gray-300 backdrop-blur-sm bg-opacity-25 ${
        open ? 'block' : 'hidden'
      }`}
      onClick={() => {
        if (clickOut && setOpen) {
          setOpen(false);
        }
      }}
    >
      <div className="relative w-full h-full">
        <div className="absolute top-[50%] left-[50%] translate-x-[-50%] translate-y-[-50%] min-w-96 min-h-80 bg-blue-950 rounded-md shadow-xl p-1">
          <IntricateFrame>{children}</IntricateFrame>
        </div>
      </div>
    </div>
  );
};

export default Modal;
