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
      className={`relative h-full w-full overflow-hidden rounded-md border-2 border-yellow-600 p-6 ${className}`}
    >
      <div className="absolute left-0 top-0 h-2 w-2 rounded-full border-2 border-yellow-600"></div>
      <div className="absolute right-0 top-0 h-2 w-2 rounded-full border-2 border-yellow-600"></div>
      <div className="absolute bottom-0 left-0 h-2 w-2 rounded-full border-2 border-yellow-600"></div>
      <div className="absolute bottom-0 right-0 h-2 w-2 rounded-full border-2 border-yellow-600"></div>
      <div className="absolute -left-1 -top-1 h-14 w-14 rotate-45 rounded-full border-l-2  border-yellow-600"></div>
      <div className="absolute -right-1 -top-1 h-14 w-14 rotate-45 rounded-full border-t-2  border-yellow-600"></div>
      <div className="absolute -bottom-1 -left-1 h-14 w-14 rotate-45 rounded-full border-b-2  border-yellow-600"></div>
      <div className="absolute -bottom-1 -right-1 h-14 w-14 rotate-45 rounded-full border-r-2  border-yellow-600"></div>
      <div className="absolute bottom-4 left-0 right-0 top-4 bg-blue-950"></div>
      <div className="absolute bottom-0 left-4 right-4 top-0 bg-blue-950"></div>
      <div className="relative z-10 h-full w-full">{children}</div>
    </div>
  );
};

export default IntricateFrame;
