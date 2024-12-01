/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx}",
    "./components/**/*.{js,ts,jsx,tsx}",
    "./app/**/*.{js,ts,jsx,tsx}",
    "./context/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    container: {
      center: "true",
      padding: "2rem",
      screens: {
        "2xl": "1400px",
      },
    },
    extend: {
      fontFamily: {
        "josefin-sans": ['"Josefin Sans"', "sans-serif"],
        montserrat: ['"Montserrat"', "sans-serif"],
        dotGothic16: ["DotGothic16", "sans-serif"],
        teko: ["Teko", "sans-serif"],
        dm: ["DM Serif Text", "serif"],
      },
      colors: {
        primary: "var(--color-primary)",
        secondary: "var(--color-secondary)",
        tertiary: "var(--color-tertiary)",
        quaternary: "var(--color-quaternary)",
        quinary: "var(--color-quinary)",
        "btn-left-bg": "#6c8df8",
        "btn-left-text": "#444",
        "btn-right-text": "#eee",
      },
      textShadow: {
        custom:
          "3px 3px 0 var(--color-secondary), 6px 6px 0 var(--color-tertiary), 9px 9px var(--color-quaternary), 12px 12px 0 var(--color-quinary)",
      },
      keyframes: {
        moveToLeft: {
          "0%": { transform: "translateX(120px)", opacity: "0" },
          "100%": { transform: "translateX(0)", opacity: "1" },
        },
        moveToRight: {
          "0%": { transform: "translateX(-120px)", opacity: "0" },
          "100%": { transform: "translateX(0)", opacity: "1" },
        },
        animateBtn: {
          "0%": { opacity: "0" },
          "100%": { opacity: "1" },
          shadows: "shadows 1.2s ease-in infinite",
          move: "move 1.2s ease-in infinite",
        },
        shadows: {
          "0%": { textShadow: "none" },
          "10%": { textShadow: "3px 3px 0 var(--color-secondary)" },
          "20%": {
            textShadow:
              "3px 3px 0 var(--color-secondary), 6px 6px 0 var(--color-tertiary)",
          },
          "30%": {
            textShadow:
              "3px 3px 0 var(--color-secondary), 6px 6px 0 var(--color-tertiary), 9px 9px var(--color-quaternary)",
          },
          "40%": {
            textShadow:
              "3px 3px 0 var(--color-secondary), 6px 6px 0 var(--color-tertiary), 9px 9px var(--color-quaternary), 12px 12px 0 var(--color-quinary)",
          },
          "50%": {
            textShadow:
              "3px 3px 0 var(--color-secondary), 6px 6px 0 var(--color-tertiary), 9px 9px var(--color-quaternary), 12px 12px 0 var(--color-quinary)",
          },
          "60%": {
            textShadow:
              "3px 3px 0 var(--color-secondary), 6px 6px 0 var(--color-tertiary), 9px 9px var(--color-quaternary), 12px 12px 0 var(--color-quinary)",
          },
          "70%": {
            textShadow:
              "3px 3px 0 var(--color-secondary), 6px 6px 0 var(--color-tertiary), 9px 9px var(--color-quaternary)",
          },
          "80%": {
            textShadow:
              "3px 3px 0 var(--color-secondary), 6px 6px 0 var(--color-tertiary)",
          },
          "90%": { textShadow: "3px 3px 0 var(--color-secondary)" },
          "100%": { textShadow: "none" },
        },
        move: {
          "0%": { transform: "translate(0px, 0px)" },
          "40%": { transform: "translate(-12px, -12px)" },
          "50%": { transform: "translate(-12px, -12px)" },
          "60%": { transform: "translate(-12px, -12px)" },
          "100%": { transform: "translate(0px, 0px)" },
        },
        fadeIn: {
          "0%": { opacity: "0" },
          "100%": { opacity: "1" },
        },
        shimmer: {
          from: {
            backgroundPosition: "0 0",
          },
          to: {
            backgroundPosition: "-200% 0",
          },
        },
        lineSlide: {
          "0%": { transform: "translateX(-20px)", opacity: "0" },
          "50%": { transform: "translateX(0px)", opacity: "1" },
          "100%": { transform: "translateX(0px)", opacity: "1" },
        },
        lineFade: {
          "0%": { opacity: "0" },
          "100%": { opacity: "1" },
        },
        "hamburger-open": {
          "0%": { transform: "translateX(-20px)", opacity: "0" },
          "50%": { transform: "translateX(0px)", opacity: "1" },
          "100%": { transform: "translateX(0px)", opacity: "1" },
        },
        "hamburger-close": {
          "0%": { transform: "translateX(-20px)", opacity: "0" },
          "50%": { transform: "translateX(0px)", opacity: "1" },
          "100%": { transform: "translateX(0px)", opacity: "1" },
        },
      },
      animation: {
        moveToLeft: "moveToLeft 2s ease",
        moveToRight: "moveToRight 2s ease",
        animateBtn: "animateBtn 1s ease",
        shadows: "shadows 1.2s ease-in infinite",
        move: "move 1.2s ease-in infinite",
        fadeIn: "fadeIn 2s ease",
        "line-slide": "lineSlide 1s ease-out",
        "line-fade": "lineFade 1s ease-out",
        "btn-left": "animateBtn 1s 1.5s backwards",
        "btn-right": "animateBtn 1s 2.3s backwards",
        "hamburger-open": "hamburger-open 0.5s ease-in-out",
        "hamburger-close": "hamburger-close 0.5s ease-in-out",
      },
      scale: {
        120: "1.2",
      },
      spacing: {
        7: "1.75rem",
        14: "3.5rem",
      },
      rotate: {
        45: "45deg",
        "-45": "-45deg",
        90: "90deg",
      },
      translate: {
        6: "1.5rem",
        "-6": "-1.5rem",
      },
    },
    screens: {
      xs: "480px",
      sm: "640px",
      md: "768px",
      lg: "1024px",
      xl: "1280px",
      "2xl": "1536px",
      770: "770px", // display Hamburger button
    },
  },
  plugins: [
    function ({ addUtilities }) {
      addUtilities(
        {
          ".text-shadow": {
            textShadow:
              "3px 3px 0 var(--color-secondary), 6px 6px 0 var(--color-tertiary), 9px 9px var(--color-quaternary), 12px 12px 0 var(--color-quinary)",
          },
        },
        ["responsive", "hover"]
      );
    },
  ],
};
