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
      className={`relative rounded-md min-w-96 w-full min-h-80 h-full p-6 border-yellow-600 border-2 ${className}`}
    >
      <div className="absolute w-2 h-2 rounded-full top-0 left-0 border-2 border-yellow-600"></div>
      <div className="absolute w-2 h-2 rounded-full top-0 right-0 border-2 border-yellow-600"></div>
      <div className="absolute w-2 h-2 rounded-full bottom-0 left-0 border-2 border-yellow-600"></div>
      <div className="absolute w-2 h-2 rounded-full bottom-0 right-0 border-2 border-yellow-600"></div>
      <div className="absolute w-14 h-14 rotate-45 -top-1 -left-1 rounded-full border-l-2  border-yellow-600"></div>
      <div className="absolute w-14 h-14 rotate-45 -top-1 -right-1 rounded-full border-t-2  border-yellow-600"></div>
      <div className="absolute w-14 h-14 rotate-45 -bottom-1 -left-1 rounded-full border-b-2  border-yellow-600"></div>
      <div className="absolute w-14 h-14 rotate-45 -bottom-1 -right-1 rounded-full border-r-2  border-yellow-600"></div>

      {children}
    </div>
  );
};

export default IntricateFrame;
