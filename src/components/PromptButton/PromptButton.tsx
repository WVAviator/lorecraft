import { HTMLAttributes } from 'react';

interface PromptButtonProps extends HTMLAttributes<HTMLButtonElement> {
  children: React.ReactNode;
}

const PromptButton: React.FC<PromptButtonProps> = ({
  children,
  className,
  ...rest
}) => {
  return (
    <button
      className={`min-w-24 text-md bg-transparent p-1 rounded-md hover:bg-blue-800 ${className}`}
      {...rest}
    >
      {children}
    </button>
  );
};

export default PromptButton;
