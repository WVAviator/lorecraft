import React, { Attributes } from 'react';

interface ContainerCarouselProps<T> {
  children: React.ReactElement<T>[];
  onChange?: (index: number) => void;
  activeItemProps?: Partial<T> & Attributes;
  inactiveItemProps?: Partial<T> & Attributes;
  sizeDiff?: number;
  spacing?: number;
}

const ContainerCarousel = <T,>({
  children,
  onChange = () => {},
  activeItemProps = {},
  inactiveItemProps = {},
  sizeDiff = 0.2,
  spacing = 0.33,
}: ContainerCarouselProps<T>) => {
  const length = React.useMemo(
    () => React.Children.count(children),
    [children]
  );
  const [selected, setSelected] = React.useState(0);

  React.useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'ArrowLeft') {
        const newIndex = (selected - 1 + length) % length;
        setSelected(newIndex);
        onChange(newIndex);
      } else if (e.key === 'ArrowRight') {
        const newIndex = (selected + 1) % length;
        setSelected(newIndex);
        onChange(newIndex);
      }
    };
    document.addEventListener('keydown', handleKeyDown);

    return () => {
      document.removeEventListener('keydown', handleKeyDown);
    };
  }, [selected, length, onChange]);

  const handleLeftArrowClick = () => {
    const newIndex = (selected - 1 + length) % length;
    setSelected(newIndex);
    onChange(newIndex);
  };

  const handleRightArrowClick = () => {
    const newIndex = (selected + 1) % length;
    setSelected(newIndex);
    onChange(newIndex);
  };

  return (
    <div className="relative flex h-full w-full items-center justify-center overflow-hidden">
      <div
        className=" absolute left-0 top-[50%] z-10 h-full min-w-[22vw] translate-y-[-50%] cursor-pointer"
        onClick={handleLeftArrowClick}
      ></div>
      <div className="perspective-1000 relative h-full w-full">
        {React.Children.map(children, (child, index) => {
          const isActive = index === selected;
          const childProps = isActive ? activeItemProps : inactiveItemProps;

          return (
            <div
              className="preserve-3d absolute h-full w-full text-center transition-transform duration-700"
              style={{
                transform: `translateX(${
                  (index - selected) * (100 * spacing)
                }%) scale(${1 - Math.abs(index - selected) * sizeDiff})`,
              }}
            >
              {React.cloneElement(child, {
                ...child.props,
                ...childProps,
              })}
            </div>
          );
        })}
      </div>
      <div
        className="absolute right-0 top-[50%] z-10 h-full min-w-[22vw] translate-y-[-50%] cursor-pointer"
        onClick={handleRightArrowClick}
      ></div>
    </div>
  );
};

export default ContainerCarousel;
