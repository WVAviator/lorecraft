/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{ts,tsx}', './index.html'],
  theme: {
    fontSize: {
      sm: '1rem',
      base: '1.25rem',
      lg: '1.5rem',
      xl: '2rem',
      '2xl': '2.5rem',
      '3xl': '3rem',
      '4xl': '4rem',
      '5xl': '5rem',
    },
    extend: {
      fontFamily: {
        amarante: ['Amarante, serif'],
        almendra: ['Almendra, serif'],
        'dancing-script': ['Dancing Script, cursive'],
        'apple-chancery': ['Apple Chancery, cursive'],
        'roboto-slab': ['Roboto Slab, serif'],
        inter: ['Inter, serif'],
        overlock: ['OverlockSC', 'serif'],
      },
      backgroundImage: {
        'stone': "url('/images/common/stone_pattern.png')"
      }
    },
  },
  safelist: [
    'page-enter',
    'page-enter-active',
    'page-exit',
    'page-exit-active',
  ],
  plugins: [],
};
