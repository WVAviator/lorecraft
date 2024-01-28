import React from 'react';
import useProcessImage from '../../hooks/useProcessImage';
import { Scene, Image } from '../../types/Game';
import BackgroundDiv from '../BackgroundDiv/BackgroundDiv';

interface SceneImageProps {
  scene: Scene | undefined;
  children: React.ReactNode;
}

const SceneImage: React.FC<SceneImageProps> = ({ scene, children }) => {
  const [fade, setFade] = React.useState(false);
  const [lastImage, setLastImage] = React.useState<Image | undefined>(
    scene?.image
  );

  React.useEffect(() => {
    setFade(true);
    setTimeout(() => {
      setLastImage(scene?.image);
      setFade(false);
    }, 500);
  }, [scene]);

  const { src, alt } = useProcessImage(lastImage);

  return (
    <BackgroundDiv image={src || undefined} alt={alt} fade={fade}>
      <div className="absolute bottom-0 left-0 right-0 top-0">
        <div className="relative h-full w-full">
          <div className="absolute left-4 top-0 rounded-b-md bg-black bg-opacity-65 px-4 py-2">
            <h1 className="text-xl">{scene?.name ?? 'Undefined Scene'}</h1>
          </div>
          {children}
        </div>
      </div>
    </BackgroundDiv>
  );
};

export default SceneImage;
