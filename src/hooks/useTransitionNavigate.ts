import React from 'react';
import { useNavigate } from 'react-router-dom';

const useTransitionNavigate = (delay: number = 500) => {
  const navigate = useNavigate();
  const [isTransitioning, setIsTransitioning] = React.useState(false);

  const navigateWithTransition = (path: string) => {
    setIsTransitioning(true);
    setTimeout(() => {
      navigate(path);
    }, delay);
  };

  return { navigateWithTransition, isTransitioning };
};

export default useTransitionNavigate;
