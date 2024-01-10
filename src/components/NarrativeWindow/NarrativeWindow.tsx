import React from 'react';
import styles from './NarrativeWindow.module.css';

interface NarrativeWindowProps {
  messages: string[];
}

const NarrativeWindow: React.FC<NarrativeWindowProps> = ({ messages }) => {
  const containerRef = React.useRef<HTMLDivElement>(null);
  React.useEffect(() => {
    containerRef.current?.scrollTo(0, containerRef.current?.scrollHeight);
  }, [messages.length]);
  return (
    <div
      ref={containerRef}
      className="flex h-full w-full flex-grow-0 flex-col gap-2 overflow-scroll scroll-smooth rounded-md border-2 border-gray-50 p-2 text-[14px]"
    >
      {messages.map((message, id) => {
        return <p key={id}>{message}</p>;
      })}
    </div>
  );
};

export default NarrativeWindow;
