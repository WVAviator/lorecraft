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
    <div ref={containerRef} className={styles.container}>
      {messages.map((message, id) => {
        return <p key={id}>{message}</p>;
      })}
    </div>
  );
};

export default NarrativeWindow;
