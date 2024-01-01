export interface SetupRequest {
  openaiApiKey?: string;
}

export interface SetupResponse {
  success: boolean;
  error?: 'file_system_error' | 'missing_openai_key';
  message?: string;
}
