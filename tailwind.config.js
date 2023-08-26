/** @type {import('tailwindcss').Config} */

const colors = require('tailwindcss/colors')

module.exports = {
  content: { 
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    colors: {
      primary: colors.indigo,
      secondary: colors.blue,
      background: colors.emerald,
      text: colors.emerald,
      link: colors.indigo,
      button: colors.cyan,
      error: colors.rose,
      success: colors.emerald,
      warning: colors.yellow,
      info: colors.slate,
      disabled: colors.slate,
    },
  },
  plugins: [require("@tailwindcss/typography")],
}
