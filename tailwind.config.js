
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
    extend: {
      fontFamily: {
        display: ['"SF Display"', '"Helvetica Neue"', 'Helvetica', 'Arial', 'sans-serif'],
        text: ['"SF Pro Text"', '"Helvetica Neue"', 'Helvetica', 'Arial', 'sans-serif'],
      },
    },
  },
  plugins: [],
}
