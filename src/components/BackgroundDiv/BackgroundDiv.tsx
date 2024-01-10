import React from 'react';

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
    <div
      className={`relative h-full w-full bg-black transition-opacity duration-300 ${
        fade ? 'opacity-0' : 'opacity-100'
      }`}
      {...rest}
    >
      {image && (
        <img
          src={image}
          alt={alt || ''}
          className="absolute left-0 top-0 h-full w-full object-cover"
        />
      )}
      {children}
    </div>
  );
};

export default BackgroundDiv;
