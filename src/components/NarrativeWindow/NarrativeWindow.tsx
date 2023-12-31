import styles from './NarrativeWindow.module.css';

interface NarrativeWindowProps {
  messages: string[];
}

const NarrativeWindow: React.FC<NarrativeWindowProps> = ({ messages }) => {
  return (
    <div className={styles.container}>
      {messages.map((message) => {
        return <p>{message}</p>;
      })}
    </div>
  );
};

export default NarrativeWindow;
