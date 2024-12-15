
// tailwind.config.js

/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: [
    "./src/**/*.{rs,html,css}",
    "./assets/**/*.css"
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
