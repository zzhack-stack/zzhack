/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "selector",
  content: ["app/**/*.{rs,html}", "entry/**/*.{rs,html}"],
  theme: {
    extend: {},
    colors: {
      black: {
        light: "#1e1e1e",
        DEFAULT: "#1e1e1e",
        700: "rgba(0, 0, 0, 0.7)",
        500: "#7a7a7a",
      },
      white: {
        light: "#fefefe",
        DEFAULT: "#fefefe",
      },
    },
  },
  plugins: [require("@tailwindcss/typography")],
};
