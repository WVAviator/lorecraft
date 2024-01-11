import pattern01 from '/images/common/spinners/ancient-pattern.png';

const LoadingSpinner = () => {
  return (
    <div>
      <img className="animate-pulse drop-shadow-sm" src={pattern01} alt="Loading..." />
    </div>
  );
};

export default LoadingSpinner;
