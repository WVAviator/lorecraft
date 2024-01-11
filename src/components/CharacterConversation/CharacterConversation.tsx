import styles from './CharacterConversation.module.css';

interface CharacterConversationProps {
  messages: CharacterMessage[];
}

const CharacterConversation: React.FC<CharacterConversationProps> = ({
  messages,
}) => {
  return (
    <div className="flex h-full w-full flex-col gap-2 overflow-scroll rounded-md border-2 border-gray-50 p-2 text-sm">
      {messages.map((message, id) => {
        if (message.is_dialog) return <p key={id}>{message.text}</p>;
        return (
          <p key={id} style={{ fontStyle: 'italic', color: '#f0f0f0' }}>
            {message.text}
          </p>
        );
      })}
    </div>
  );
};

export default CharacterConversation;
