import { RouterProvider, createHashRouter } from 'react-router-dom';
import SplashLoadingScreen from './screens/SplashLoadingScreen';
import MainMenuScreen from './screens/MainMenuScreen';
import GameScreen from './screens/GameScreen';
import GameGenerationScreen from './screens/GameGenerationScreen';
import GameSelectionScreen from './screens/GameSelectionScreen';
import NarrativeScreen from './screens/NarrativeScreen';
import GameProvider from './context/GameProvider';
import GameMenuScreen from './screens/GameMenuScreen';
import GameStateProvider from './context/GameStateProvider';
import ScreenContainer from './screens/ScreenContainer';
import { createRef } from 'react';
import GenerateOptionsScreen from './screens/GenerateOptionsScreen';

export const routes = [
  {
    path: '/',
    element: <SplashLoadingScreen />,
    nodeRef: createRef<HTMLDivElement>(),
  },
  {
    path: '/mainmenu',
    element: <MainMenuScreen />,
    nodeRef: createRef<HTMLDivElement>(),
  },
  {
    path: '/select-game',
    element: <GameSelectionScreen />,
    nodeRef: createRef<HTMLDivElement>(),
  },
  {
    path: '/generate-options',
    element: <GenerateOptionsScreen />,
    nodeRef: createRef<HTMLDivElement>(),
  },
  {
    path: '/generate-game',
    element: <GameGenerationScreen />,
    nodeRef: createRef<HTMLDivElement>(),
  },
  {
    path: '/game',
    element: <GameScreen />,
    nodeRef: createRef<HTMLDivElement>(),
  },
  {
    path: '/narrative',
    element: <NarrativeScreen />,
    nodeRef: createRef<HTMLDivElement>(),
  },
  {
    path: '/gamemenu',
    element: <GameMenuScreen />,
    nodeRef: createRef<HTMLDivElement>(),
  },
];

const router = createHashRouter([
  {
    path: '/',
    element: <ScreenContainer />,
    children: routes.map(({ path, element }) => ({
      path,
      element,
    })),
  },
]);

function App() {
  return (
    <GameProvider>
      <GameStateProvider>
        <RouterProvider router={router} />
      </GameStateProvider>
    </GameProvider>
  );
}

export default App;
