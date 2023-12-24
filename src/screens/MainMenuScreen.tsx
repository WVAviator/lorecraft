import React from 'react';
import background from '../assets/images/menu/background.png';
import AbsoluteContainer from '../components/AbsoluteContainer/AbsoluteContainer';
import BackgroundDiv from '../components/BackgroundDiv/BackgroundDiv';
import MenuList from '../components/MenuList/MenuList';
import MenuListItem from '../components/MenuListItem/MenuListItem';
import { useNavigate } from 'react-router-dom';

interface MenuOption {
  label: string;
  onSelect: () => void;
}

const BG_ALT_DESC =
  'an old scroll containing menu options unrolled across a stone surface next to quills, ink, and stamps';

const MainMenuScreen = () => {
  const [selected, setSelected] = React.useState(0);
  const navigate = useNavigate();
  const menuOptions: MenuOption[] = React.useMemo(
    () => [
      {
        label: 'New Game',
        onSelect: () => {
          navigate('/gamesetup');
        },
      },
      {
        label: 'Load Game',
        onSelect: () => console.log('load game'),
      },
      {
        label: 'Options',
        onSelect: () => console.log('options'),
      },
      {
        label: 'Quit',
        onSelect: () => console.log('quit'),
      },
    ],
    []
  );

  React.useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'ArrowUp') {
        setSelected(
          (prev) => (prev - 1 + menuOptions.length) % menuOptions.length
        );
      } else if (e.key === 'ArrowDown') {
        setSelected((prev) => (prev + 1) % menuOptions.length);
      } else if (e.key === 'Enter') {
        menuOptions[selected].onSelect();
      }
    };

    window.addEventListener('keydown', handleKeyDown);

    return () => {
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, [menuOptions, selected]);

  return (
    <BackgroundDiv image={background} alt={BG_ALT_DESC}>
      <AbsoluteContainer left="32%" right="32%" top="36%" bottom="5%">
        <MenuList>
          {menuOptions.map((option, index) => (
            <MenuListItem
              key={option.label}
              selected={selected === index}
              onMouseEnter={() => setSelected(index)}
              onClick={option.onSelect}
            >
              {option.label}
            </MenuListItem>
          ))}
        </MenuList>
      </AbsoluteContainer>
    </BackgroundDiv>
  );
};

export default MainMenuScreen;
