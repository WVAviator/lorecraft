import React from 'react';
import { IoBookOutline } from 'react-icons/io5';
import { IoMdClose } from 'react-icons/io';
import PlayerEntry from '../PlayerEntry/PlayerEntry';

interface NarrativeWindowProps {
  messages: string[];
  sendMessage: (message: string) => void;
}

const NarrativeWindow: React.FC<NarrativeWindowProps> = ({
  messages,
  sendMessage,
}) => {
  const [playerInput, setPlayerInput] = React.useState<string>('');
  const [open, setOpen] = React.useState<boolean>(false);

  const containerRef = React.useRef<HTMLDivElement>(null);
  React.useEffect(() => {
    containerRef.current?.scrollTo(0, containerRef.current?.scrollHeight);
  }, [messages.length]);
  return (
    <div
      className={`absolute bottom-2 right-0 top-14 flex w-[50vw] items-center rounded-l-md bg-black bg-opacity-75 py-4 pr-4 transition-transform duration-200 ease-in-out ${
        open ? 'translate-x-0' : 'translate-x-[calc(100%-2rem)]'
      }`}
    >
      <button
        onClick={() => setOpen((open) => !open)}
        className="mx-2 h-full w-16"
      >
        {open ? <IoMdClose /> : <IoBookOutline />}
      </button>
      <div className="flex h-full w-full flex-col items-center justify-between gap-4">
        <div
          ref={containerRef}
          className="flex h-full w-full flex-grow-0 flex-col gap-2 overflow-scroll scroll-smooth rounded-md border-2 border-gray-50 p-2 text-[14px]"
        >
          {messages.map((message, id) => {
            return <p key={id}>{message}</p>;
          })}
        </div>
        <PlayerEntry
          value={playerInput}
          onChange={(e) => {
            if (playerInput.length >= 497) return;
            setPlayerInput(e.target.value);
          }}
          onSubmit={() => {
            sendMessage(playerInput);
            setPlayerInput('');
          }}
          placeholder="What do you want to do?"
          rows={2}
        />
      </div>
    </div>
  );
};

export default NarrativeWindow;
