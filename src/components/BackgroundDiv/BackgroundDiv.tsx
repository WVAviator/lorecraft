import React from 'react';
import styles from './BackgroundDiv.module.css';

interface BackgroundDivProps {
  children: React.ReactNode;
  image: string;
  alt: string;
  fade?: boolean;
  fadeTime?: number;
  onlyFadeImage?: boolean;
}

const BackgroundDiv: React.FC<BackgroundDivProps> = ({
  children,
  image,
  alt,
  fade = false,
  onlyFadeImage = false,
}) => {
  const fadeClass = fade ? styles.fadeOut : styles.fadeIn;

  return (
    <div className={`${styles.container} ${onlyFadeImage ? '' : fadeClass}`}>
      <img
        src={image}
        alt={alt}
        className={`${styles.image} ${onlyFadeImage ? fadeClass : ''}`}
      />
      {children}
    </div>
  );
};

export default BackgroundDiv;
