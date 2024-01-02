import BackgroundDiv from '../components/BackgroundDiv/BackgroundDiv';
import background from '/images/splash/background.png';
import FlexContainer from '../components/FlexContainer/FlexContainer';
import LoadingSpinner from '../components/LoadingSpinner/LoadingSpinner';
import React from 'react';
import useTransitionNavigate from '../hooks/useTransitionNavigate';
import { invoke } from '@tauri-apps/api';
import { isSetupResponse } from '../types/Setup';
import ApiKeyEntry from '../components/ApiKeyEntry/ApiKeyEntry';
import AlertDialog from '../components/AlertDialog/AlertDialog';

const BG_ALT_DESC =
  'A mystical leatherbound book embedded with a glowing blue gem surrounded by intricate patterns rests partially buried in sand in a desert valley surrounding by sharp mountain peaks with a glowing blue and pink aurora in the night sky';

const SplashLoadingScreen = () => {
  const { navigateWithTransition, isTransitioning } =
    useTransitionNavigate(1000);

  const [setupRetries, setSetupRetries] = React.useState(0);
  const [request, setRequest] = React.useState<Partial<Request>>({});
  const [apiKeyError, setApiKeyError] = React.useState(false);
  const [networkError, setNetworkError] = React.useState(false);
  const [fileSystemError, setFileSystemError] = React.useState(false);

  React.useEffect(() => {
    const setupApp = async () => {
      const minWait = new Promise((res) => setTimeout(res, 3000));
      const setup = invoke('setup', { request });

      try {
        await Promise.all([setup, minWait]);
        navigateWithTransition('/mainmenu');
      } catch (error) {
        console.error(error);

        if (!isSetupResponse(error)) return;
        await new Promise((res) => setTimeout(res, 1000));

        if (
          error.error === 'missing_openai_key' ||
          error.error === 'bad_openai_key'
        ) {
          setApiKeyError(true);
        } else if (error.error === 'connection_failed') {
          setNetworkError(true);
        } else if (error.error === 'file_system_error') {
          setFileSystemError(true);
        }
      }
    };

    setupApp();
  }, [request, setupRetries]);

  const handleProvideApiKey = (apiKey: string) => {
    setRequest((previousRequest) => ({
      ...previousRequest,
      openai_api_key: apiKey,
    }));
    setSetupRetries((retries) => retries + 1);
  };

  return (
    <BackgroundDiv image={background} alt={BG_ALT_DESC} fade={isTransitioning}>
      <ApiKeyEntry
        open={apiKeyError}
        setOpen={setApiKeyError}
        onSubmit={handleProvideApiKey}
      />
      <AlertDialog
        open={networkError}
        title="Network Error"
        message="Could not connect to OpenAI servers. Are you connected to the internet?"
        actions={[
          {
            title: 'Quit',
            onSelect: () => {
              window.close();
            },
          },
          {
            title: 'Try Again',
            onSelect: () => {
              setSetupRetries((retries) => retries + 1);
              setNetworkError(false);
            },
          },
        ]}
      />
      <AlertDialog
        open={fileSystemError}
        title="File Error"
        message="Could not access your filesystem to save game files. Please verify your local app data directory exists and allows read and write permission for Lorecraft."
        actions={[
          {
            title: 'Quit',
            onSelect: () => {
              window.close();
            },
          },
          {
            title: 'Try Again',
            onSelect: () => {
              setSetupRetries((retries) => retries + 1);
              setNetworkError(false);
            },
          },
        ]}
      />
      <FlexContainer
        alignItems="flex-end"
        width="100%"
        height="100%"
        padding="0.5rem"
      >
        <LoadingSpinner />
      </FlexContainer>
    </BackgroundDiv>
  );
};

export default SplashLoadingScreen;
