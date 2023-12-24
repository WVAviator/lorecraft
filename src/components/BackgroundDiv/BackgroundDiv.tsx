import React from 'react';
import styles from './BackgroundDiv.module.css';

interface BackgroundDivProps {
  children: React.ReactNode;
  image: string;
  alt: string;
  fade?: boolean;
}

const BackgroundDiv: React.FC<BackgroundDivProps> = ({
  children,
  image,
  alt,
  fade = false,
}) => {
  return (
    <div
      className={`${styles.container} ${fade ? styles.fadeOut : styles.fadeIn}`}
    >
      <img src={image} alt={alt} className={styles.image} />
      {children}
    </div>
  );
};

export default BackgroundDiv;
