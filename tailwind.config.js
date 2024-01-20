/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  darkMode:"class",
  content: ["./src/**/*.{rs,html,css}","./public/tailwind.css"],
  theme: {
    fontFamily:{
      sans: ["outfit"],
      serif: ["outfit"]
    },
    extend: {},
  },
  plugins: [],
}

