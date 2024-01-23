/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  darkMode:"class",
  content: ["./src/**/*.{rs,html,css}","./dist/**/*.html"],
  theme: {
    fontFamily:{
      sans: ["outfit"],
      serif: ["outfit"]
    },
    extend: {},
  },
  plugins: [],
}

