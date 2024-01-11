import React from 'react';

interface TextAreaProps
  extends React.DetailedHTMLProps<
    React.TextareaHTMLAttributes<HTMLTextAreaElement>,
    HTMLTextAreaElement
  > {}

const TextArea = React.forwardRef<HTMLTextAreaElement, TextAreaProps>(
  ({ className, ...rest }, ref) => {
    return (
      <textarea
        ref={ref}
        className={`block w-full rounded-md border-2 border-gray-50 bg-transparent p-2 text-sm placeholder:text-sm focus:border-blue-500 focus:ring-blue-500 ${className}`}
        {...rest}
      ></textarea>
    );
  }
);

export default TextArea;
