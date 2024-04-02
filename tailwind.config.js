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
        slideRL: {
          '0%': { transform: 'translateX(100%)'},
          '100%': {transform: 'translateX(0)'},
        },
        irregularShaking:{
          '0%': {transform:'translateX(0);translateY(0)'},
          '25%': {transform:'translateX(-35%);translateY(12%)'},
          '50%': {transform:'translateX(-15%);translateY(35%)'},
          '75%': {transform:'translateX(-15%);translateY(18%)'},
          '100%': {transform:'translateX(-42%);translateY(32%)'},
        }
      },
      animation:{
        'wordBlink': 'blink 0.75s infinite step-end',
        'irregularShaking': 'irregularShaking 1s linear infinite alternate-reverse',
        'slideFromR2L': 'slideRL 900ms linear'
      }
    },
  },
  plugins: [],
}

