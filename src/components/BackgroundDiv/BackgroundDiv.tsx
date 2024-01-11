import React from 'react';
import useRandomPan from '../../hooks/useRandomPan';

interface BackgroundDivProps extends React.HTMLAttributes<HTMLDivElement> {
  children: React.ReactNode;
  image?: string;
  alt?: string;
  fade?: boolean;
  fadeTime?: number;
  onlyFadeImage?: boolean;
  randomPan?: boolean;
}

const BackgroundDiv: React.FC<BackgroundDivProps> = ({
  children,
  image,
  alt,
  fade = false,
  onlyFadeImage = false,
  randomPan = false,
  ...rest
}) => {
  const randomPanStyle = useRandomPan(randomPan, 10000, [image]);
  return (
    <div className={`relative h-full w-full bg-black `} {...rest}>
      {image && (
        <div
          className="absolute bottom-0 left-0 right-0 top-0"
          style={{ ...randomPanStyle }}
        >
          <img
            src={image}
            alt={alt || ''}
            className={`  h-full w-full object-cover transition-opacity  duration-300 ${
              fade ? 'opacity-0' : 'opacity-100'
            }`}
          />
        </div>
      )}
      {children}
    </div>
  );
};

export default BackgroundDiv;
