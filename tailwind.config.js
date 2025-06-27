/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",
    "./index.html",
    "./dist/**/*.html",
  ],
  theme: {
    extend: {
      colors: {
        'terminal': {
          'bg': '#1e1e1e',
          'header': '#2d2d30',
          'border': '#3e3e42',
          'text': '#d4d4d4',
          'prompt': '#4ec9b0',
          'success': '#4ec9b0',
          'error': '#f44747',
          'info': '#569cd6',
          'placeholder': '#6a6a6a',
        }
      },
      fontFamily: {
        'mono': ['Courier New', 'monospace'],
      }
    },
  },
  plugins: [
    require('@tailwindcss/typography'),
  ],
}