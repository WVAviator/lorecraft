/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{ts,tsx}", "./index.html"],
  theme: {
    extend: {
      fontFamily: {
        amarante: ["Amarante, serif"],
        almendra: ["Almendra, serif"],
        "dancing-script": ["Dancing Script, cursive"],
        "apple-chancery": ["Apple Chancery, cursive"],
        "roboto-slab": ["Roboto Slab, serif"],
      }
    },
  },
  plugins: [],
}

