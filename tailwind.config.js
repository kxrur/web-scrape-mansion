/** @type {import('tailwindcss').Config} */
export default {
  purge: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  theme: {
    extend: {
      colors: {
        background: '#19323C',
        foreground: '#F3F7F0',
        secondary: '#8C5E58',
        accent: '#A93F55',
        highlight: '#AB8476',
      },
    },
  },
  plugins: [],
}

