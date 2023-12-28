import BackgroundDiv from '../components/BackgroundDiv/BackgroundDiv';
import background from '/images/splash/background.png';
import FlexContainer from '../components/FlexContainer/FlexContainer';
import LoadingSpinner from '../components/LoadingSpinner/LoadingSpinner';
import React from 'react';
import useTransitionNavigate from '../hooks/useTransitionNavigate';

const BG_ALT_DESC =
  'A mystical leatherbound book embedded with a glowing blue gem surrounded by intricate patterns rests partially buried in sand in a desert valley surrounding by sharp mountain peaks with a glowing blue and pink aurora in the night sky';

const SplashLoadingScreen = () => {
  const { navigateWithTransition, isTransitioning } =
    useTransitionNavigate(1000);
  React.useEffect(() => {
    setTimeout(() => {
      navigateWithTransition('/mainmenu');
    }, 3000);
  }, []);

  return (
    <BackgroundDiv image={background} alt={BG_ALT_DESC} fade={isTransitioning}>
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
