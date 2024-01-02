import React from 'react';
import Underline from '../Underline/Underline';
import styles from './MenuListItem.module.css';

interface MenuListItemProps {
  children: React.ReactNode;
  selected?: boolean;
  onClick?: () => void;
  onMouseEnter?: () => void;
  onMouseLeave?: () => void;
}

const MenuListItem: React.FC<MenuListItemProps> = ({
  children,
  selected = false,
  onClick,
  onMouseEnter,
  onMouseLeave,
}) => {
  const textRef = React.useRef<HTMLParagraphElement>(null);

  return (
    <li
      className={styles.listitem}
      onClick={() => {
        onClick && onClick();
      }}
      onMouseEnter={() => {
        onMouseEnter && onMouseEnter();
      }}
      onMouseLeave={() => {
        onMouseLeave && onMouseLeave();
      }}
    >
      <p ref={textRef} className={styles.listitemtext}>
        {children}
      </p>
      <div
        style={{
          width: textRef.current
            ? `${textRef.current?.offsetWidth + 16}px`
            : 'auto',
        }}
        className={styles.underline}
      >
        <Underline visible={selected} />
      </div>
    </li>
  );
};

export default MenuListItem;
