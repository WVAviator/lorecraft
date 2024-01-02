import styles from './GameMenuListItem.module.css';

interface GameMenuListItemProps {
  children: React.ReactNode;
  onClick?: () => void;
}

const GameMenuListItem: React.FC<GameMenuListItemProps> = ({
  children,
  onClick = () => {},
}) => {
  return (
    <li onClick={onClick} className={styles.listitem}>
      {children}
    </li>
  );
};

export default GameMenuListItem;
