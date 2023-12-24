interface AbsoluteContainerProps {
  children: React.ReactNode;
  top?: string;
  left?: string;
  right?: string;
  bottom?: string;
  width?: string;
  height?: string;
  backgroundColor?: string;
  className?: string;
}

const AbsoluteContainer: React.FC<AbsoluteContainerProps> = ({
  children,
  top = 'auto',
  left = 'auto',
  right = 'auto',
  bottom = 'auto',
  width = 'auto',
  height = 'auto',
  backgroundColor = 'transparent',
  className = '',
}) => {
  return (
    <div
      style={{
        position: 'absolute',
        top,
        left,
        right,
        bottom,
        width,
        height,
        backgroundColor,
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
