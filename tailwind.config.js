/** @type {import('tailwindcss').Config} */


module.exports = {
  content: { 
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
  },
  plugins: [require("@tailwindcss/typography", "@prettier-plugin-tailwindcss")],
}
