import pattern01 from '/images/common/spinners/ancient-pattern.png';
import styles from './LoadingSpinner.module.css';

const LoadingSpinner = () => {
  return (
    <div>
      <img className={styles.spinner} src={pattern01} alt="Loading..." />
    </div>
  );
};

export default LoadingSpinner;
