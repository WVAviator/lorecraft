export interface SetupRequest {
  openai_api_key?: string;
}

export interface SetupResponse {
  success: boolean;
  error?:
    | 'file_system_error'
    | 'missing_openai_key'
    | 'connection_failed'
    | 'bad_openai_key';
  message?: string;
}

export const isSetupResponse = (error: any): error is SetupResponse => {
  return (
    error &&
    typeof error.error === 'string' &&
    typeof error.message === 'string'
  );
};
