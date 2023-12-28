import React from 'react';
import BackgroundDiv from '../BackgroundDiv/BackgroundDiv';

interface CycledBackgroundProps {
  children: React.ReactNode;
  images: {
    src: string;
    alt: string;
  }[];
  transitionTime?: number;
  cycleTime?: number;
  play?: boolean;
}

const CycledBackground: React.FC<CycledBackgroundProps> = ({
  children,
  images,
  cycleTime = 5000,
  play = true,
}) => {
  const [imageIndex, setImageIndex] = React.useState(0);
  const [fade, setFade] = React.useState(false);
  const [onlyFadeImage, setOnlyFadeImage] = React.useState(true);

  React.useEffect(() => {
    if (!play) {
      setFade(true);
      setOnlyFadeImage(false);
      return;
    }
    const timeout = setTimeout(() => {
      setFade(true);
      setTimeout(() => {
        setImageIndex((imageIndex + 1) % images.length);
        setFade(false);
      }, 1000);
    }, cycleTime);

    return () => clearTimeout(timeout);
  }, [images.length, imageIndex, play, cycleTime]);

  return (
    <BackgroundDiv
      image={images[imageIndex].src}
      alt={images[imageIndex].alt}
      fade={fade}
      onlyFadeImage={onlyFadeImage}
    >
      {children}
    </BackgroundDiv>
  );
};

export default CycledBackground;
