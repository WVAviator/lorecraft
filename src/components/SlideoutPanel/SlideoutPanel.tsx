import React from 'react';
import { IoIosArrowBack } from 'react-icons/io';
import { GrClose } from 'react-icons/gr';

interface SlideoutPanelProps {
  children: React.ReactNode;
  tabContentOpen?: React.ReactNode;
  tabContentClosed?: React.ReactNode;
}

const SlideoutPanel: React.FC<SlideoutPanelProps> = ({
  children,
  tabContentOpen = <GrClose />,
  tabContentClosed = <IoIosArrowBack />,
}) => {
  const [open, setOpen] = React.useState(false);
  return (
    <div
      className={`debug absolute bottom-0 left-0 top-0 z-30 h-full min-w-24 bg-gray-900 transition-transform duration-200 ease-in-out ${
        open ? 'translate-x-0' : '-translate-x-[100%]'
      }`}
    >
      <div className="relative">
        <button
          className="absolute right-0 top-4 h-12 w-6 translate-x-[100%] cursor-pointer rounded-r-md bg-gray-900 p-2"
          onClick={() => setOpen((open) => !open)}
        >
          {React.cloneElement(open ? tabContentOpen : tabContentClosed)}
        </button>
        {children}
      </div>
    </div>
  );
};

export default SlideoutPanel;
