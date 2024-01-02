import styles from './MenuList.module.css';

interface MenuListProps {
  children: React.ReactNode;
  gap?: string;
  alignItems?: string;
}

const MenuList: React.FC<MenuListProps> = ({
  children,
  gap = '1rem',
  alignItems = 'center',
}) => {
  return (
    <ul className={styles.list} style={{ gap, alignItems }}>
      {children}
    </ul>
  );
};

export default MenuList;
