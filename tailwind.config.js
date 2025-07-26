/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs", "./index.html", "./dist/**/*.html"],
  theme: {
    extend: {
      colors: {
        terminal: {
          bg: "var(--terminal-bg)",
          header: "var(--terminal-header)",
          border: "var(--terminal-border)",
          text: "var(--terminal-text)",
          prompt: "var(--terminal-prompt)",
          success: "var(--terminal-success)",
          error: "var(--terminal-error)",
          info: "var(--terminal-info)",
          placeholder: "#6a6a6a",
        },
      },
      fontFamily: {
        mono: ["CaskaydiaCove Nerd Font", "Cascadia Code", "Consolas", "monospace"],
      },
    },
  },
  plugins: [],
};
