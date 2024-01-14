import React from 'react';

interface SliderProps {
  range: [number, number];
  value: number;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
}

const Slider: React.FC<SliderProps> = ({ range, value, onChange }) => {
  return (
    <div className="relative mb-6">
      <input
        min={range[0]}
        max={range[1]}
        step="0.1"
        type="range"
        value={value}
        onChange={onChange}
        className="h-2 w-full cursor-pointer appearance-none rounded-lg bg-gray-200 dark:bg-gray-700"
      />
      <span class="absolute -bottom-6 start-0 text-sm text-gray-100 ">
        {range[0]}
      </span>
      <span className="absolute -bottom-6 start-1/2 text-sm text-gray-100">
        {value}
      </span>
      <span class="absolute -bottom-6 end-0 text-sm text-gray-100 ">
        {range[1]}
      </span>
    </div>
  );
};

export default Slider;
