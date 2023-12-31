import { RouterProvider, createHashRouter } from 'react-router-dom';
import SplashLoadingScreen from './screens/SplashLoadingScreen';
import SetupScreen from './screens/SetupScreen';
import MainMenuScreen from './screens/MainMenuScreen';
import SettingsScreen from './screens/SettingsScreen';
import GameScreen from './screens/GameScreen';
import GameGenerationScreen from './screens/GameGenerationScreen';
import GameSelectionScreen from './screens/GameSelectionScreen';
import NarrativeScreen from './screens/NarrativeScreen';
import GameProvider from './context/GameProvider';
import GameMenuScreen from './screens/GameMenuScreen';

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
    path: '/select-game',
    element: <GameSelectionScreen />,
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
  {
    path: '/narrative',
    element: <NarrativeScreen />,
  },
  {
    path: '/gamemenu',
    element: <GameMenuScreen />,
  },
]);

function App() {
  return (
    <GameProvider>
      <RouterProvider router={router} />
    </GameProvider>
  );
}

export default App;
