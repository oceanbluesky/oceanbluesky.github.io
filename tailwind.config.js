/** @type {import('tailwindcss').Config} */

module.exports = {
  content: [
    "./src/**/*.{rs,html}",  // Includes Rust and HTML files in src
    "./public/**/*.html",     // Includes HTML files in public directory
  ],
  theme: {
    extend: {},
  },
  plugins: [],
};
