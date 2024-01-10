import React from 'react';
import Underline from '../Underline/Underline';

interface MenuListItemProps {
  children: React.ReactNode;
  selected?: boolean;
  onClick?: () => void;
  onMouseEnter?: () => void;
  onMouseLeave?: () => void;
}

const MenuListItem: React.FC<MenuListItemProps> = ({
  children,
  selected = false,
  onClick,
  onMouseEnter,
  onMouseLeave,
}) => {
  const textRef = React.useRef<HTMLParagraphElement>(null);

  return (
    <a className="group">
      <li
        className="w-full text-center flex flex-col items-center cursor-pointer"
        onClick={() => {
          onClick && onClick();
        }}
        onMouseEnter={() => {
          onMouseEnter && onMouseEnter();
        }}
        onMouseLeave={() => {
          onMouseLeave && onMouseLeave();
        }}
      >
        <p
          ref={textRef}
          className="font-dancing-script text-3xl text-gray-800 drop-shadow-sm group-hover:drop-shadow-md group-focus:drop-shadow-md"
        >
          {children}
        </p>
        <div className="-mt-[0.35rem] overflow-hidden flex justify-center items-center ">
          <Underline visible={selected} />
        </div>
      </li>
    </a>
  );
};

export default MenuListItem;
