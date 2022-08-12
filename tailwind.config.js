/** @type {import('tailwindcss').Config} */
const colors = require('tailwindcss/colors');

module.exports = {
  content: [
    "./src/**/*.{tsx, jsx, js, ts}",
  ],
  theme: {
    extend: {},
    colors: {
      black: colors.black,
      blue: colors.blue,
      yellow: colors.yellow,
      red: colors.red,
    }
  },
  plugins: [],
}