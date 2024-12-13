
/** @type {import('tailwindcss').Config} */

module.exports = {
  content: [
    "./src/**/*.{rs,html,css}",
    "./dist/**/*.html",
  ],
  theme: {
    extend: {
      fontFamily: {
        display: ['"SF Display"', '"Helvetica Neue"', 'Helvetica', 'Arial', 'sans-serif'],
        text: ['"SF Pro Text"', '"Helvetica Neue"', 'Helvetica', 'Arial', 'sans-serif'],
      },
    },
  },
  plugins: [],
};
