import React from 'react';
import TextArea from '../TextArea/TextArea';

interface PlayerEntryProps
  extends Omit<
    React.DetailedHTMLProps<
      React.TextareaHTMLAttributes<HTMLTextAreaElement>,
      HTMLTextAreaElement
    >,
    'ref'
  > {
  onSubmit?: () => void;
}

const PlayerEntry: React.FC<PlayerEntryProps> = ({
  onSubmit = () => {},
  onKeyDown = () => {},
  ...rest
}) => {
  const inputRef = React.useRef<HTMLTextAreaElement>(null);

  React.useEffect(() => {
    const observer = new IntersectionObserver((entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting && inputRef.current) {
          inputRef.current.focus();
        } else {
          inputRef.current?.blur();
        }
      });
    });

    if (inputRef.current) {
      observer.observe(inputRef.current);
    }

    return () => {
      if (inputRef.current) {
        observer.unobserve(inputRef.current);
      }
    };
  }, []);

  return (
    <TextArea
      ref={inputRef}
      onKeyDown={(e) => {
        if (onKeyDown) onKeyDown(e);
        if (e.key === 'Enter' && !e.shiftKey) {
          e.preventDefault();
          onSubmit();
        }
      }}
      {...rest}
    />
  );
};

export default PlayerEntry;
