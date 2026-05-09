/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {
      colors: {
        smarthr: {
          blue: "#00c4cc",
          "product-main": "#0077c7",
          danger: "#e01e5a",
          warning: "#ffcc17",
          "text-black": "#23221e",
          "text-grey": "#706d65",
          "text-disabled": "#c1bdb7",
          stone01: "#f8f7f6",
          stone02: "#edebe8",
          stone03: "#aaa69f",
          stone04: "#4e4c49",
          border: "#d6d3d0",
          "over-background": "#f2f1f0",
        },
      },
      fontFamily: {
        yugothic: [
          'AdjustedYuGothic',
          '"Yu Gothic"',
          'YuGothic',
          '"Hiragino Sans"',
          'sans-serif',
        ],
      },
    },
  },
  plugins: [],
};
