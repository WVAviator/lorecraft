import { useRef } from 'react';
import { useLocation, useOutlet } from 'react-router-dom';
import {
  CSSTransition,
  SwitchTransition,
  TransitionGroup,
} from 'react-transition-group';
import { routes } from '../App';

const ScreenContainer = () => {
  const outlet = useOutlet();
  const location = useLocation();
  const { nodeRef } =
    routes.find((route) => route.path === location.pathname) ?? {};
  return (
    <SwitchTransition>
      <CSSTransition
        key={location.pathname}
        nodeRef={nodeRef}
        timeout={250}
        classNames={'page'}
        unmountOnExit
      >
        {() => (
          <div ref={nodeRef} className="h-full w-full">
            {outlet}
          </div>
        )}
      </CSSTransition>
    </SwitchTransition>
  );
};

export default ScreenContainer;
