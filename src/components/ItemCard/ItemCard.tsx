import useProcessImage from '../../hooks/useProcessImage';
import { Item } from '../../types/Game';

interface ItemCardProps {
  item: Item;
}

const ItemCard: React.FC<ItemCardProps> = ({ item }) => {
  const { src, alt } = useProcessImage(item.image);

  return (
    <div className="flex max-w-36 cursor-pointer flex-col items-center justify-center gap-2 rounded-md bg-yellow-950 p-2 shadow-lg transition-transform duration-300  ease-in-out hover:translate-y-[-4px] hover:shadow-xl">
      <h3 className="text-wrap text-center font-overlock-sc text-sm">
        {item.name}
      </h3>
      <img
        src={src}
        alt={alt}
        className="shadow-iiner rounded-md object-cover"
      />
    </div>
  );
};

export default ItemCard;
