import pattern01 from '../../assets/images/common/pattern.png';
import styles from './LoadingSpinner.module.css';

const LoadingSpinner = () => {
  return (
    <div>
      <img className={styles.spinner} src={pattern01} alt="Loading..." />
    </div>
  );
};

export default LoadingSpinner;
