import AbsoluteContainer from '../AbsoluteContainer/AbsoluteContainer';
import MenuList from '../MenuList/MenuList';
import styles from './GameMenu.module.css';

interface GameMenuProps {
  children: React.ReactNode;
}

const GameMenu: React.FC<GameMenuProps> = ({ children }) => {
  return (
    <AbsoluteContainer
      right="0"
      bottom="3rem"
      padding="0 3rem 0 10%"
      className={styles.container}
    >
      <MenuList alignItems="flex-end">{children}</MenuList>
    </AbsoluteContainer>
  );
};

export default GameMenu;
