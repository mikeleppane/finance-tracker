/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: [
      "*.html",
      "./src/**/*.rs",
    ],
  },
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
      },
    },
  },
  plugins: [],
}
