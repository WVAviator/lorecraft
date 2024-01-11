import React from 'react';
import MenuList from '../MenuList/MenuList';

interface GameMenuProps {
  children: React.ReactNode;
}

const GameMenu: React.FC<GameMenuProps> = ({ children }) => {
  const [load, setLoad] = React.useState(true);
  React.useEffect(() => {
    setTimeout(() => setLoad(false), 1000);
  }, []);
  return (
    <div
      className={`absolute bottom-12 right-0 bg-gradient-to-l from-gray-800 to-transparent px-12 py-0 transition-all duration-1000 ${
        load ? 'translate-x-[100%] opacity-0' : 'translate-x-0 opacity-100'
      }`}
    >
      <MenuList alignItems="flex-end">{children}</MenuList>
    </div>
  );
};

export default GameMenu;
