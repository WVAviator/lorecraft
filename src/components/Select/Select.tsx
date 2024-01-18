import React from 'react';

interface SelectProps {
  options: string[];
  value: string;
  onChange: (e: React.ChangeEvent<HTMLSelectElement>) => void;
}
const Select: React.FC<SelectProps> = ({ options, value, onChange }) => {
  return (
    <select
      onChange={(e) => onChange(e)}
      className="block w-full rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900 focus:border-blue-500 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 dark:focus:border-blue-500 dark:focus:ring-blue-500"
    >
      {options.map((option) => (
        <option selected={value === option} value={option}>
          {option}
        </option>
      ))}
    </select>
  );
};

export default Select;
