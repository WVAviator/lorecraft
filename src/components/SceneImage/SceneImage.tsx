import useProcessImage from '../../hooks/useProcessImage';
import { Scene } from '../../types/Game';
import styles from './SceneImage.module.css';

interface SceneImageProps {
  scene: Scene | undefined;
}

const SceneImage: React.FC<SceneImageProps> = ({ scene }) => {
  if (!scene) return null;
  const { src, alt } = useProcessImage(scene.image);
  return (
    <div className={styles.container}>
      <div className={styles.title_container}>
        <h2 className={styles.title}>{scene.name}</h2>
      </div>
      <div className={styles.image_container}>
        <img src={src} alt={alt} className={styles.image} />
      </div>
      <div className={styles.narrative_container}>
        <p className={styles.narrative}>{scene.narrative}</p>
      </div>
    </div>
  );
};

export default SceneImage;
