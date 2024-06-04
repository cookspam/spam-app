/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: [
    // include all rust, html and css files in the src directory
    "./src/**/*.{rs,html,css}",
    // include all html files in the output (dist) directory
    "./dist/**/*.html",
  ],
  darkMode: 'selector',
  plugins: [],
  theme: {
    colors: ({ colors }) => ({
      ...colors,
      transparent: colors.transparent,
      white: '#f5f5f7',
      real_white:'#00000',
      black: '#1d1d1f',
      green: {
        400: '#5EDCA7',
        500: '#3498DB',
        600: '#BDC3C7',
        700: '#00a36e',
      },
      gray: {
        100: '#e8e8ea',
        200: '#d6d6d8',
        300: '#b0b0b2',
        700: '#707071',
        800: '#464648',
        900: '#313133',
      },
      yellow: {
        500:'#ffeb3b',
        600: '#fdd835',
      },
      pink:{
        100: '#f8bbd0',
        200: '#ff80ab',
        800: '#f9a825'
      },
      amber:{
        200: '#fde68a',
        500: '#f59e0b',

      },
      teal: {
        500: '#14b8a6',
        700: '#0f766e',
        900: '#134e4a'
      }

    }),
    extend: {
      fontFamily: {
        mono: ['GeistMono', 'ui-monospace', 'monospace'],
        sans: ['Geist', 'ui-sans-serif', 'system-ui'],
        serif: ['ui-serif'],
        hero: ['RoobertTRIAL', 'ui-sans-serif', 'system-ui']
      },
      transitionProperty: {
        'height': 'height'
      },
    },
  },
}
