/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: [
    // include all rust, html and css files in the src directory
    "./src/**/*.{rs,html,css}",
    // include all html files in the output (dist) directory
    "./dist/**/*.html",
  ],
  theme: {
    colors: ({ colors }) => ({
      ...colors,
      transparent: colors.transparent,
      white: '#f5f5f7',
      black: '#1d1d1f',
      green: {
        500: '#00cc8a',
        600: '#00b87c',
        700: '#00a36e',
      },
      gray: {
        100: '#e8e8ea',
        200: '#d6d6d8',
        300: '#b0b0b2',
      },
    }),
    lineHeight: {
        'relaxed': '1.3',
    },
    extend: {
      transitionProperty: {
        'height': 'height'
      },
    },
  },
  plugins: [],
}