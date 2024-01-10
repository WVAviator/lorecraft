import React from 'react';
import styles from './BackgroundDiv.module.css';

interface BackgroundDivProps extends React.HTMLAttributes<HTMLDivElement> {
  children: React.ReactNode;
  image?: string;
  alt?: string;
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
  style,
  ...rest
}) => {
  return (
    <div className={`${styles.container}`} {...rest}>
      {image && (
        <img src={image} alt={alt || ''} className={`${styles.image}`} />
      )}
      {children}
    </div>
  );
};

export default BackgroundDiv;
