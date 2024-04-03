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
        },
        wait:{
          '0%':{'background-image':'linear-gradient(to right top,rgb(239 246 255) 20%,rgb(249 250 251) 40%,#f3f4f6)', 'opacity': '1'},
          '10%':{'background-image':'linear-gradient(to right top,#f3f4f6 20%,rgb(239 246 255) 40%,rgb(249 250 251))', 'opacity': '1'},
          '20%':{'background-image':'linear-gradient(to right top,rgb(249 250 251) 20%,#f3f4f6 40%,rgb(239 246 255))', 'opacity': '1'},
          '30%':{'background-image':'linear-gradient(to right top,rgb(239 246 255) 20%,rgb(249 250 251) 40%,#f3f4f6)', 'opacity': '1'},
          '40%':{'background-image':'linear-gradient(to right top,#f3f4f6 20%,rgb(239 246 255) 40%,rgb(249 250 251))', 'opacity': '1'},
          '50%':{'background-image':'linear-gradient(to right top,rgb(249 250 251) 20%,#f3f4f6 40%,rgb(239 246 255))', 'opacity': '.5'},
          '60%':{'background-image':'linear-gradient(to right top,rgb(239 246 255) 20%,rgb(249 250 251) 40%,#f3f4f6)', 'opacity': '1'},
          '70%':{'background-image':'linear-gradient(to right top,#f3f4f6 20%,rgb(239 246 255) 40%,rgb(249 250 251))', 'opacity': '1'},
          '80%':{'background-image':'linear-gradient(to right top,rgb(249 250 251) 20%,#f3f4f6 40%,rgb(239 246 255))', 'opacity': '1'},
          '90%':{'background-image':'linear-gradient(to right top,rgb(239 246 255) 20%,rgb(249 250 251) 40%,#f3f4f6)', 'opacity': '1'},
          '100%':{'background-image':'linear-gradient(to right top,#f3f4f6 20%,rgb(239 246 255) 40%,rgb(249 250 251))', 'opacity': '1'},
        }
      },
      animation:{
        'wordBlink': 'blink 0.75s infinite step-end',
        'down': 'down 1s 1 linear',
        'wait': 'wait 2s infinite'
      }
    },
  },
  plugins: [],
}

