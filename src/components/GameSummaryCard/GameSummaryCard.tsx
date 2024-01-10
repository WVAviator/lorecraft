import { convertFileSrc } from '@tauri-apps/api/tauri';
import { Game } from '../../types/Game';
import cardBack from '/images/common/square_card_back.png';
import React from 'react';

interface GameSummaryCardProps {
  game: Game;
  faceDown?: boolean;
  onClick?: () => void;
}

const GameSummaryCard: React.FC<GameSummaryCardProps> = ({
  game,
  faceDown = false,
  onClick = () => {},
}) => {
  const yRotation = faceDown ? 180 : 0;
  const xRotation = faceDown ? 1 : -1;

  React.useEffect(() => {
    const enterListener = (e: KeyboardEvent) => {
      if (e.key === 'Enter' && faceDown === false) {
        onClick();
      }
    };
    window.addEventListener('keydown', enterListener);
    return () => {
      window.removeEventListener('keydown', enterListener);
    };
  }, [onClick, faceDown]);

  return (
    <div className="flex h-full w-full items-center justify-center">
      <div className="perspective-1000 h-[400px] w-[400px]">
        <div
          className="preserve-3d relative h-full w-full cursor-pointer text-center transition-all duration-300 ease-linear"
          style={{
            transform: `rotateY(${yRotation}deg) rotateX(${xRotation}deg)`,
          }}
          onClick={() => {
            if (!faceDown) {
              onClick();
            }
          }}
        >
          <div className="backface-hidden shadow-inner-lg absolute h-full w-full overflow-hidden rounded-md bg-[#b1835b]">
            <img
              className="pointer-events-none h-full w-full select-none object-cover"
              src={convertFileSrc(game.cover_art.src)}
              alt={game.cover_art.alt}
            />
            <div className="shadow-inner-lg [*]:pointer-events-none [*]:select-none absolute bottom-0 left-0 w-full bg-stone text-left text-[14px] drop-shadow-sm before:absolute before:bottom-0 before:left-0 before:right-0 before:top-0 before:bg-black before:bg-opacity-25">
              <div className="relative h-full w-full p-3">
                <h2 className="font-overlock-sc text-lg">{game.name}</h2>
                <p>{game.summary.description}</p>
              </div>
            </div>
          </div>
          <div className="rotate-y-180 backface-hidden shadow-inner-lg absolute h-full w-full overflow-hidden rounded-md bg-transparent">
            <img
              src={cardBack}
              alt="intricate arcane stone pattern card back"
            />
          </div>
        </div>
      </div>
    </div>
  );
};

export default GameSummaryCard;

// const card = document.querySelector('.card-inner');
// const rect = card.getBoundingClientRect();
// let hover = false;
// let active = false;
// card.addEventListener('mouseenter', () => {
//   hover = true;
// });
// card.addEventListener('mousedown', () => {
//   active = true;
//   card.style.transform = `rotateX(0deg) rotateY(180deg)`
// });
// document.addEventListener('mouseup', () => {
//   active = false;
//   card.style.transform = `rotateX(0deg) rotateY(0deg)`
// })
// card.addEventListener('mouseleave', () => {
//   hover = false;
//   card.style.transform = `rotateX(0deg) rotateY(0deg)`
// });
// card.addEventListener('mousemove', (event) => {
//     if (!hover || active) return;
//     const rY = 50 * (event.offsetX / rect.width - 0.5);
//     const rX = -50 * (event.offsetY / rect.height - 0.5);
//     // const rY = 5 * rect.width + rect.width / 2;
//     card.style.transform = `rotateX(${rX}deg) rotateY(${rY}deg)`
// });
