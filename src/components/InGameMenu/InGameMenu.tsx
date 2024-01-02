import React from 'react';
import styles from './InGameMenu.module.css';
import { IconButton, Tooltip } from '@mui/material';

interface MenuItem {
  icon: React.ReactElement;
  tooltip: string;
  onClick: () => void;
}

interface InGameMenuProps {
  menuItems: MenuItem[];
}

const InGameMenu: React.FC<InGameMenuProps> = ({ menuItems }) => {
  return (
    <div className={styles.container}>
      {menuItems.map((menuItem) => {
        return (
          <div className={styles.icon_container} onClick={menuItem.onClick}>
            <Tooltip title={menuItem.tooltip}>
              <IconButton>
                {React.cloneElement(menuItem.icon, {
                  className: styles.icon,
                })}
              </IconButton>
            </Tooltip>
          </div>
        );
      })}
    </div>
  );
};

export default InGameMenu;
