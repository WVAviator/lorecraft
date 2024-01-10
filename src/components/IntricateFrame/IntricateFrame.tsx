interface IntricateFrameProps {
  children: React.ReactNode;
  className?: string;
}

const IntricateFrame: React.FC<IntricateFrameProps> = ({
  children,
  className = '',
}) => {
  return (
    <div
      className={`relative h-full min-h-80 w-full min-w-96 rounded-md border-2 border-yellow-600 p-6 ${className}`}
    >
      <div className="absolute left-0 top-0 h-2 w-2 rounded-full border-2 border-yellow-600"></div>
      <div className="absolute right-0 top-0 h-2 w-2 rounded-full border-2 border-yellow-600"></div>
      <div className="absolute bottom-0 left-0 h-2 w-2 rounded-full border-2 border-yellow-600"></div>
      <div className="absolute bottom-0 right-0 h-2 w-2 rounded-full border-2 border-yellow-600"></div>
      <div className="absolute -left-1 -top-1 h-14 w-14 rotate-45 rounded-full border-l-2  border-yellow-600"></div>
      <div className="absolute -right-1 -top-1 h-14 w-14 rotate-45 rounded-full border-t-2  border-yellow-600"></div>
      <div className="absolute -bottom-1 -left-1 h-14 w-14 rotate-45 rounded-full border-b-2  border-yellow-600"></div>
      <div className="absolute -bottom-1 -right-1 h-14 w-14 rotate-45 rounded-full border-r-2  border-yellow-600"></div>

      {children}
    </div>
  );
};

export default IntricateFrame;
