import useProcessImage from '../../hooks/useProcessImage';
import { Scene } from '../../types/Game';

interface SceneImageProps {
  scene: Scene | undefined;
}

const SceneImage: React.FC<SceneImageProps> = ({ scene }) => {
  if (!scene) return null;
  const { src, alt } = useProcessImage(scene.image);
  return (
    <div className="flex h-[100vh] flex-col bg-blue-950 p-2">
      <div className="h-12 pb-2">
        <h2 className="flex-shrink-0 flex-grow-0 font-overlock-sc text-xl">
          {scene.name}
        </h2>
      </div>
      <div className="w-full flex-1 overflow-hidden rounded-md shadow-md">
        <img
          src={src}
          alt={alt}
          className="h-full w-full object-cover [object-position:50%_85%]"
        />
      </div>
      <div className="max-h-[30%] overflow-scroll pt-2">
        <p className="flex-shrink-0 flex-grow-0 text-[14px]">
          {scene.narrative}
        </p>
      </div>
    </div>
  );
};

export default SceneImage;
