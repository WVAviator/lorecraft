import React from 'react';

interface MenuItem {
  icon: React.ReactElement;
  tooltip: string;
  onClick: () => void;
}

interface InGameMenuProps {
  menuItems: MenuItem[];
}

const InGameMenu: React.FC<InGameMenuProps> = ({ menuItems }) => {
  const [hoverIndex, setHoverIndex] = React.useState<number>(-1);
  return (
    <div className="flex h-12 w-full flex-row-reverse items-center gap-4">
      {menuItems.map((menuItem, i) => {
        return (
          <div
            className="flex h-8 w-8 cursor-pointer items-center justify-center"
            onMouseEnter={() => setHoverIndex(i)}
            onMouseLeave={() => setHoverIndex(-1)}
          >
            <button onClick={menuItem.onClick}>
              {React.cloneElement(menuItem.icon, {
                className: 'text-lg hover:text-gray-400',
              })}
            </button>
          </div>
        );
      })}
      {hoverIndex !== -1 && <p>{menuItems[hoverIndex].tooltip}</p>}
    </div>
  );
};

export default InGameMenu;
