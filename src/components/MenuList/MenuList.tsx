import styles from './MenuList.module.css';

interface MenuListProps {
  children: React.ReactNode;
}

const MenuList: React.FC<MenuListProps> = ({ children }) => {
  return <ul className={styles.list}>{children}</ul>;
};

export default MenuList;
