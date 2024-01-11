import { HTMLAttributes } from 'react';

interface PromptButtonProps
  extends React.DetailedHTMLProps<
    React.ButtonHTMLAttributes<HTMLButtonElement>,
    HTMLButtonElement
  > {
  children: React.ReactNode;
}

const PromptButton: React.FC<PromptButtonProps> = ({
  children,
  className,
  disabled,
  ...rest
}) => {
  return (
    <button
      className={`min-w-24 text-md bg-transparent p-1 rounded-md ${
        disabled
          ? 'text-gray-600'
          : 'text-gray-50 hover:bg-blue-300 hover:bg-opacity-20'
      } ${className}`}
      disabled={disabled}
      {...rest}
    >
      {children}
    </button>
  );
};

export default PromptButton;
