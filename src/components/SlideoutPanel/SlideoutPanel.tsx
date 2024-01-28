import React from 'react';
import { IoIosArrowBack } from 'react-icons/io';
import { GrClose } from 'react-icons/gr';
import IntricateFrame from '../IntricateFrame/IntricateFrame';

interface SlideoutPanelProps {
  children: React.ReactNode;
  tabContentOpen?: React.ReactElement;
  tabContentClosed?: React.ReactElement;
}

const SlideoutPanel: React.FC<SlideoutPanelProps> = ({
  children,
  tabContentOpen = <GrClose />,
  tabContentClosed = <IoIosArrowBack />,
}) => {
  const [open, setOpen] = React.useState(false);
  return (
    <div
      className={`absolute bottom-0 left-0 top-0 z-30 h-full min-w-24 bg-black bg-opacity-75 transition-transform duration-200 ease-in-out ${
        open ? '-translate-x-4' : '-translate-x-[100%]'
      }`}
    >
      <div className="relative h-full">
        <button
          className="absolute right-0 top-20 h-16 w-8 translate-x-[100%] cursor-pointer rounded-r-md bg-black bg-opacity-75 p-2"
          onClick={() => setOpen((open) => !open)}
        >
          {open
            ? React.cloneElement(tabContentOpen!)
            : React.cloneElement(tabContentClosed!)}
        </button>
        <IntricateFrame>
          <div className="h-full pl-4">{children}</div>
        </IntricateFrame>
      </div>
    </div>
  );
};

export default SlideoutPanel;
