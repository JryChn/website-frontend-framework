/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  // this will generate all class
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
        down:{
          '0%': {'transform':'translateY(-100%) rotate(-8deg)'},
          '40%': {'transform':'translateY(0) rotate(-8deg)'},
          '50%': {'transform':'translateY(-10%) rotate(-8deg)'},
          '55%': {'transform':'translateY(-8%) rotate(-6deg)'},
          '60%': {'transform':'translateY(-5%) rotate(-4deg)'},
          '65%': {'transform':'rotate(-4deg)'},
          '70%': {'transform':'translateY(-2%) rotate(-3deg)'},
          '75%': {'transform':'rotate(-2deg)'},
          '90%': {'transform':'translateY(-1%) rotate(2deg)'},
          '100%': {'transform':'rotate(0deg)'},
        }
      },
      animation:{
        'wordBlink': 'blink 0.75s infinite step-end',
        'down': 'down 1s 1 linear'
      }
    },
  },
  plugins: [],
}

