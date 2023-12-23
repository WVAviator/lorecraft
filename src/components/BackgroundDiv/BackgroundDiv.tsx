import styles from './BackgroundDiv.module.css';

interface BackgroundDivProps {
  children: React.ReactNode;
  image: string;
  alt: string;
}

const BackgroundDiv: React.FC<BackgroundDivProps> = ({
  children,
  image,
  alt,
}) => {
  return (
    <div className={styles.container}>
      <img src={image} alt={alt} className={styles.image} />
      {children}
    </div>
  );
};

export default BackgroundDiv;
