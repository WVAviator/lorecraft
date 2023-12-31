import { convertFileSrc } from '@tauri-apps/api/tauri';
import { Image } from '../types/Game';

const useProcessImage = (image: Image | undefined): Image => {
  const src = convertFileSrc(image?.src || '');
  const alt = image?.alt || '';

  return { src, alt };
};

export default useProcessImage;
