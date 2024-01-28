import React from 'react';
import { Scene } from '../../types/Game';
import { CiTextAlignLeft } from 'react-icons/ci';
import { IoMdClose } from 'react-icons/io';

interface SceneDescriptionProps {
  scene: Scene | undefined;
}

const SceneDescription: React.FC<SceneDescriptionProps> = ({ scene }) => {
  const [open, setOpen] = React.useState(false);

  React.useEffect(() => {
    setTimeout(() => {
      setOpen(true);
    }, 1000);
    setOpen(true);
  }, [scene]);

  return (
    <div
      className={`absolute bottom-4 left-0 w-[75vw] rounded-r-md bg-black bg-opacity-75 py-4 pl-4 transition duration-200 ease-in-out ${
        open ? 'translate-x-0' : '-translate-x-[calc(100%-2rem)]'
      }`}
    >
      <div className="relative flex h-full w-full items-center">
        <p className="">{scene?.narrative ?? 'Undefined Scene'}</p>
        <button
          onClick={() => setOpen((open) => !open)}
          className="mx-2 h-full w-16"
        >
          {open ? <IoMdClose /> : <CiTextAlignLeft />}
        </button>
      </div>
    </div>
  );
};

export default SceneDescription;
