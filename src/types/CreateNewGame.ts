import { Game } from './Game';

export interface CreateNewGameRequest {
  prompt: string;
}

export interface CreateNewGameResponse {
  success: boolean;
  game?: Game;
  error?: 'file_system_error' | 'setup_error' | 'game_generation_error';
  message?: string;
}
