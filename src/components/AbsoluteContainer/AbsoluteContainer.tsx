interface AbsoluteContainerProps {
  children: React.ReactNode;
  top?: string;
  left?: string;
  right?: string;
  bottom?: string;
  width?: string;
  height?: string;
  backgroundColor?: string;
  padding?: string;
  className?: string;
}

const AbsoluteContainer: React.FC<AbsoluteContainerProps> = ({
  children,
  className = '',
  ...rest
}) => {
  return (
    <div
      style={{
        position: 'absolute',
        ...rest,
      }}
      className={className}
    >
      <div style={{ position: 'relative', width: '100%', height: '100%' }}>
        {children}
      </div>
    </div>
  );
};

export default AbsoluteContainer;
