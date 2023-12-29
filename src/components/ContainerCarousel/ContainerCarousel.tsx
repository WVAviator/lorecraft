import React, { Attributes } from 'react';
import styles from './ContainerCarousel.module.css';

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
    <div className={styles.container}>
      <div className={styles.left_arrow} onClick={handleLeftArrowClick}></div>
      <div className={styles.carousel}>
        {React.Children.map(children, (child, index) => {
          const isActive = index === selected;
          const childProps = isActive ? activeItemProps : inactiveItemProps;

          return (
            <div
              className={styles.carousel_item}
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
      <div className={styles.right_arrow} onClick={handleRightArrowClick}></div>
    </div>
  );
};

export default ContainerCarousel;
