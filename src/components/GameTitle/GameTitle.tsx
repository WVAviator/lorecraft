import styles from './GameTitle.module.css';

interface GameTitleProps {
  children: React.ReactNode;
}

const GameTitle: React.FC<GameTitleProps> = ({ children }) => {
  return <h1 className={styles.heading}>{children}</h1>;
};

export default GameTitle;
