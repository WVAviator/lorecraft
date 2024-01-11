import React from 'react';
import useGameContext from './useGameContext';
import { Item } from '../types/Game';

const useInventory = (inventoryList: string[]) => {
  const { game } = useGameContext();

  const [inventory, setInventory] = React.useState<Item[]>([]);

  React.useEffect(() => {
    if (!game) return;

    const items = inventoryList.map((itemName) => {
      const item = game.items.find((i) => i.name === itemName);
      if (!item) {
        return {
          name: itemName,
          description: itemName,
          image: {
            src: '/images/common/item-placeholder.png',
            alt: 'Item Image Placeholder',
          },
        } as Item;
      }

      return item;
    });

    setInventory(items);
  }, [inventoryList]);

  return inventory;
};

export default useInventory;
