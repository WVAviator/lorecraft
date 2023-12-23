import BackgroundDiv from '../components/BackgroundDiv/BackgroundDiv';
import background from '../assets/images/splash/background.png';

const BG_ALT_DESC =
  'A mystical leatherbound book embedded with a glowing blue gem surrounded by intricate patterns rests partially buried in sand in a desert valley surrounding by sharp mountain peaks with a glowing blue and pink aurora in the night sky';

const SplashLoadingScreen = () => {
  console.log('img: ' + background);
  return (
    <BackgroundDiv image={background} alt={BG_ALT_DESC}>
      Test
    </BackgroundDiv>
  );
};

export default SplashLoadingScreen;
