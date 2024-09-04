/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "selector",
  content: ["app/**/*.{rs,html}", "entry/**/*.{rs,html}"],
  theme: {
    extend: {
      borderRadius: {
        "4xl": "30px",
        "5xl": "38px",
        "6xl": "42px",
      },
    },
    colors: {
      orange: {
        DEFAULT: "#FFBE76",
      },
      gray: {
        500: "rgba(156,156,156,0.5)",
        200: "rgba(217,217,217,0.2)",
        900: "rgba(138,143,152,0.9)",
        450: "rgba(175,177,180,0.45)",
        600: "rgba(28,28,32,0.6)",
        100: "rgba(128,132,140,0.1)",
      },
      black: {
        dark: "#F7F8F8",
        DEFAULT: "#1e1e1e",
        400: "rgba(0,0,0,0.4)",
        900: "#0A0A0A",
      },
      white: {
        dark: "#2B2B2F",
        DEFAULT: "#ffffff",
        200: "#E9E9E9",
        600: "rgba(255,255,255,0.6)",
      },
    },
  },
  plugins: [require("@tailwindcss/typography")],
};
