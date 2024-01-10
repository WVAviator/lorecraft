import React from 'react';

interface GameTitleProps {
  children: React.ReactNode;
}

const GameTitle: React.FC<GameTitleProps> = ({ children }) => {
  const [load, setLoad] = React.useState(true);
  React.useEffect(() => {
    setTimeout(() => setLoad(false), 500);
  }, []);
  return (
    <h1
      className={`font-almendra text-3xl drop-shadow-md ${
        load ? '-translate-x-6 opacity-0' : 'translate-x-0 opacity-100'
      } transition-all duration-1000`}
    >
      {children}
    </h1>
  );
};

export default GameTitle;
