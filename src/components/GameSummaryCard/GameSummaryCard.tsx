import { convertFileSrc } from '@tauri-apps/api/tauri';
import { Game } from '../../types/Game';
import styles from './GameSummaryCard.module.css';
import cardBack from '/images/common/card_back.png';
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
  const pivotDisabled = faceDown;
  const [xPivot, setXPivot] = React.useState(0);
  const [yPivot, setYPivot] = React.useState(0);

  const handlePointerMove = (event: React.MouseEvent<HTMLDivElement>) => {
    if (pivotDisabled) return;

    const { clientX, clientY } = event;
    const { left, top, width, height } =
      event.currentTarget.getBoundingClientRect();

    const offsetX = clientX - left;
    const offsetY = clientY - top;

    const rY = 50 * (offsetX / width - 0.5);
    const rX = 50 * (offsetY / height - 0.5);

    setXPivot(rX);
    setYPivot(rY);
  };

  const handlePointerLeave = () => {
    setXPivot(0);
    setYPivot(0);
  };

  const yRotation = yPivot + (faceDown ? 180 : 0);
  const xRotation = xPivot * (faceDown ? 1 : -1);

  return (
    <div className={styles.container}>
      <div className={styles.card}>
        <div
          className={styles.card_inner}
          style={{
            transform: `rotateY(${yRotation}deg) rotateX(${xRotation}deg)`,
          }}
          onClick={onClick}
          onPointerMove={handlePointerMove}
          onPointerLeave={handlePointerLeave}
        >
          <div className={styles.front}>
            <img
              // src={convertFileSrc(game.cover_art.src)}
              src="/images/splash/background.png"
              alt={game.cover_art.alt}
            />
          </div>
          <div className={styles.back}>
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
