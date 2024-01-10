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
    <ul
      className="list-none flex flex-col items-center px-4 w-full"
      style={{ gap, alignItems }}
    >
      {children}
    </ul>
  );
};

export default MenuList;
