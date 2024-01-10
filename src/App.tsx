import { RouterProvider, createHashRouter } from 'react-router-dom';
import SplashLoadingScreen from './screens/SplashLoadingScreen';
import MainMenuScreen from './screens/MainMenuScreen';
import GameScreen from './screens/GameScreen';
import GameGenerationScreen from './screens/GameGenerationScreen';
import GameSelectionScreen from './screens/GameSelectionScreen';
import NarrativeScreen from './screens/NarrativeScreen';
import GameProvider from './context/GameProvider';
import GameMenuScreen from './screens/GameMenuScreen';
import { ThemeProvider, createTheme } from '@mui/material';
import GameStateProvider from './context/GameStateProvider';
import ScreenContainer from './screens/ScreenContainer';
import { createRef } from 'react';

const theme = createTheme({
  palette: {
    mode: 'dark',
  },
  typography: {
    fontFamily: 'Amarante, serif',
  },
});

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
        <ThemeProvider theme={theme}>
          <RouterProvider router={router} />
        </ThemeProvider>
      </GameStateProvider>
    </GameProvider>
  );
}

export default App;
