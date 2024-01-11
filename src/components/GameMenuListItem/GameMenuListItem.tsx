interface GameMenuListItemProps {
  children: React.ReactNode;
  onClick?: () => void;
}

const GameMenuListItem: React.FC<GameMenuListItemProps> = ({
  children,
  onClick = () => {},
}) => {
  return (
    <li
      onClick={onClick}
      className="cursor-pointer font-overlock-sc text-lg drop-shadow-md transition-colors hover:text-gray-400"
    >
      {children}
    </li>
  );
};

export default GameMenuListItem;
