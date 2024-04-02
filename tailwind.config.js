/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  // this will generate all class
  safelist: [
    {
      pattern: /./, // the "." means "everything"
    },
  ],
  darkMode:"class",
  content: ["./src/**/*.{rs,html,css}","./dist/**/*.html"],
  theme: {
    fontFamily:{
      sans: ["outfit"],
      serif: ["outfit"]
    },
    extend: {
      keyframes: {
        blink: {
          '50%': { 'border-color': 'transparent'},
        },
      },
      animation:{
        'wordBlink': 'blink 0.75s infinite step-end'
      }
    },
  },
  plugins: [],
}

