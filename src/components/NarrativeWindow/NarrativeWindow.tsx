import styles from './NarrativeWindow.module.css';

interface NarrativeWindowProps {
  messages: string[];
}

const NarrativeWindow: React.FC<NarrativeWindowProps> = ({ messages }) => {
  return (
    <div className={styles.container}>
      {messages.map((message, id) => {
        return <p key={id}>{message}</p>;
      })}
    </div>
  );
};

export default NarrativeWindow;
