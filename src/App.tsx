import { RouterProvider, createHashRouter } from 'react-router-dom';
import SplashLoadingScreen from './screens/SplashLoadingScreen';
import SetupScreen from './screens/SetupScreen';
import MainMenuScreen from './screens/MainMenuScreen';
import SettingsScreen from './screens/SettingsScreen';
import GameScreen from './screens/GameScreen';
import GameGenerationScreen from './screens/GameGeneration';

const router = createHashRouter([
  {
    path: '/',
    element: <SplashLoadingScreen />,
  },
  {
    path: '/setup',
    element: <SetupScreen />,
  },
  {
    path: '/mainmenu',
    element: <MainMenuScreen />,
  },
  {
    path: '/settings',
    element: <SettingsScreen />,
  },
  {
    path: '/generate-game',
    element: <GameGenerationScreen />,
  },
  {
    path: '/game',
    element: <GameScreen />,
  },
]);

function App() {
  return <RouterProvider router={router} />;
}

export default App;
