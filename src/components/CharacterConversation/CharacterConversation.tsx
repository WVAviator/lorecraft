import React from 'react';

interface CharacterConversationProps {
  messages: CharacterMessage[];
}

const CharacterConversation: React.FC<CharacterConversationProps> = ({
  messages,
}) => {
  const containerRef = React.useRef<HTMLDivElement>(null);

  React.useEffect(() => {
    if (containerRef.current) {
      containerRef.current.scrollTop = containerRef.current.scrollHeight;
    }
  }, [messages]);

  return (
    <div
      ref={containerRef}
      className="flex h-96 w-full flex-col gap-2 overflow-y-scroll text-wrap rounded-md border-2 border-gray-50 p-2 text-sm"
    >
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
