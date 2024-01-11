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
        className="flex w-full cursor-pointer flex-col items-center text-center"
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
          className="w-full font-dancing-script text-3xl text-gray-800 drop-shadow-sm group-hover:drop-shadow-md group-focus:drop-shadow-md"
        >
          {children}
        </p>
        <div className="-mt-[0.35rem] flex items-center justify-center overflow-hidden ">
          <Underline visible={selected} />
        </div>
      </li>
    </a>
  );
};

export default MenuListItem;
