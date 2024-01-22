import React from 'react';

interface MusicProps extends React.MediaHTMLAttributes<HTMLAudioElement> {
  volume?: number;
}

const Music: React.FC<MusicProps> = ({
  volume = 0.1,
  autoPlay = true,
  ...rest
}) => {
  const musicRef = React.useRef<HTMLAudioElement>(null);

  React.useEffect(() => {
    if (!musicRef.current) return;

    musicRef.current.volume = volume;
    musicRef.current.play();
  }, [volume]);

  return <audio ref={musicRef} autoPlay={autoPlay} {...rest} />;
};

export default Music;
