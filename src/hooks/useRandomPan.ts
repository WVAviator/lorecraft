import React from 'react';

const useRandomPan = (
  enabled: boolean = true,
  duration: number = 8000,
  dependencies: any[] = []
) => {
  const [load, setLoad] = React.useState(true);
  React.useEffect(() => {
    setTimeout(() => setLoad(false), 25);
    return () => setLoad(true);
  }, [...dependencies]);

  const defaultStyle = {
    transform: 'translateX(0px) translateY(0px) scale(1)',
    transition: `transform ${duration}ms ease-in-out`,
  };

  if (!enabled) {
    return defaultStyle;
  }

  const scalePercent = Math.random() * 0.2;

  const initialX = (Math.random() * scalePercent - scalePercent / 2) * 100;
  const initialY = (Math.random() * scalePercent - scalePercent / 2) * 100;
  const initialScale = 1 + scalePercent;

  const targetScalePercent = Math.max(
    0,
    Math.random() * 0.1 - 0.05 + scalePercent
  );

  const targetX =
    (Math.random() * targetScalePercent - targetScalePercent / 2) * 100;
  const targetY =
    (Math.random() * targetScalePercent - targetScalePercent / 2) * 100;
  const targetScale = 1 + targetScalePercent;

  const initialStyle = {
    transform: `translateX(${initialX}%) translateY(${initialY}%) scale(${initialScale})`,
    transition: 'none',
  };

  const targetStyle = {
    transform: `translateX(${targetX}%) translateY(${targetY}%) scale(${targetScale})`,
    transition: `transform ${duration}ms ease-in-out`,
  };

  return load ? initialStyle : targetStyle;
};

export default useRandomPan;
