import React from 'react';
import useInventory from '../../hooks/useInventory';
import ItemCard from '../ItemCard/ItemCard';

interface InventoryListProps {
  inventory: string[];
}

const InventoryList: React.FC<InventoryListProps> = ({ inventory }) => {
  const items = useInventory(inventory);

  return (
    <div className="debug flex h-full flex-col items-center gap-6">
      <h2 className="font-overlock-sc text-lg">Inventory</h2>

      <ul className="no-scrollbar flex h-full flex-1 flex-col items-center gap-4 overflow-y-scroll">
        {items.map((item) => (
          <ItemCard key={item.id} item={item} />
        ))}
      </ul>
    </div>
  );
};

export default InventoryList;
