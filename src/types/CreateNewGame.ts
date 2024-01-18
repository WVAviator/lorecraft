import { Game } from './Game';

export interface CreateNewGameRequest {
  prompt: string;
  text_content_setting?: ContentSetting;
  image_content_setting?: ContentSetting;
  temperature_setting?: string;
  resume_previous?: string;
}

export type ContentSetting = 'minimum' | 'moderate' | 'high';

export interface CreateNewGameResponse {
  success: boolean;
  game?: Game;
  error?: 'file_system_error' | 'setup_error' | 'game_generation_error';
  message?: string;
}
