import React from 'react';
import BackgroundDiv from '../components/BackgroundDiv/BackgroundDiv';
import GeneralPrompt from '../components/GeneralPrompt/GeneralPrompt';
import TextInput from '../components/TextInput/TextInput';
import Slider from '../components/Slider/Slider';
import { CreateNewGameRequest } from '../types/CreateNewGame';
import Select from '../components/Select/Select';
import { useNavigate } from 'react-router-dom';
import useIncompleteGame from '../hooks/useIncompleteGame';
import AlertDialog from '../components/AlertDialog/AlertDialog';

interface GenerateStep {
  title: string;
  description: string;
  setField: keyof CreateNewGameRequest;
  entryType: 'text' | 'select' | 'slider';
  selectOptions?: string[];
  sliderRange?: [number, number];
  default: string;
}

const steps: GenerateStep[] = [
  {
    title: 'Game Prompt',
    description:
      'Provide a short description of the type of text adventure game you would like to generate. Leave this blank for something completely random.',
    setField: 'prompt',
    entryType: 'text',
    default: '',
  },
  {
    title: 'Narrative Quality',
    description:
      'This setting determines the quality of the generated text content in the game. Lower settings reduce quality, but save on API costs.',
    setField: 'text_content_setting',
    entryType: 'select',
    selectOptions: ['low', 'moderate', 'high'],
    default: 'moderate',
  },
  {
    title: 'Image Quality',
    description:
      'This determines the quality of the images generated for the game. Higher settings use more advanced models and options for the highest quality, but also the highest cost.',
    setField: 'image_content_setting',
    entryType: 'select',
    selectOptions: ['low', 'moderate', 'high'],
    default: 'moderate',
  },
  {
    title: 'Temperature',
    description:
      'Temperature sets the creativity level of the model. Higher temperatures will result in more creative and abstract gameplay, and lower temperatures will provide more stable and predictable games.',
    setField: 'temperature_setting',
    entryType: 'slider',
    sliderRange: [0, 2],
    default: '0.8',
  },
];

const GenerateOptionsScreen = () => {
  const [stepIndex, setStepIndex] = React.useState(0);
  const [request, setRequest] = React.useState<CreateNewGameRequest>({
    prompt: '',
  });
  const [value, setValue] = React.useState('');
  const [open, setOpen] = React.useState(false);

  const navigate = useNavigate();

  const { incompleteGame, clearIncompleteGame } = useIncompleteGame();

  React.useEffect(() => {
    setTimeout(() => {
      setOpen(true);
    }, 500);
  }, []);

  React.useEffect(() => {
    if (Object.keys(request).length === 4) {
      console.log('Formed request: ', request);
      navigate('/generate-game', { state: { request } });
    }
  }, [request]);

  const inputTypes = {
    text: () => (
      <TextInput value={value} onChange={(e) => setValue(e.target.value)} />
    ),
    select: () => (
      <Select
        options={steps[stepIndex].selectOptions || []}
        value={value}
        onChange={(e) => setValue(e.target.value)}
      />
    ),
    slider: () => (
      <Slider
        range={steps[stepIndex].sliderRange || [0, 2]}
        value={Number(value)}
        onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
          setValue(Number(e.target.value).toPrecision(2))
        }
      />
    ),
  };

  const handleSubmit = () => {
    setRequest((request) => {
      return {
        ...request,
        [steps[stepIndex].setField]: value,
      };
    });
    setOpen(false);

    if (stepIndex === steps.length - 1) {
      return;
    }

    setValue(steps[stepIndex + 1].default);
    setStepIndex(stepIndex + 1);

    setTimeout(() => {
      setOpen(true);
    }, 500);
  };

  return (
    <BackgroundDiv image="images/menu/arcane_stone.png">
      <GeneralPrompt
        title={steps[stepIndex].title}
        description={steps[stepIndex].description}
        open={open}
        setOpen={() => {}}
        onSubmit={handleSubmit}
      >
        {inputTypes[steps[stepIndex].entryType]()}
      </GeneralPrompt>
      <AlertDialog
        open={!!incompleteGame}
        title="Incomplete Game"
        message="Your last attempt to generate a game was interrupted. Would you like to resume generating?"
        setOpen={() => {}}
        actions={[
          {
            title: 'Discard',
            onSelect: () => {
              clearIncompleteGame();
            },
          },
          {
            title: 'Resume',
            onSelect: () => {
              navigate('/generate-game', {
                state: {
                  request: { prompt: '', resume_previous: incompleteGame },
                },
              });
            },
          },
        ]}
      />
    </BackgroundDiv>
  );
};

export default GenerateOptionsScreen;
