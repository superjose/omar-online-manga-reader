/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "jit",
  content: [
    "./src/**/*.rs",
    "./index.html",
    "./src/**/*.html",
    "./src/**/*.css",
  ],
  theme: {
    extend: {
      colors: {
        darkness: "#242729",
        "darkness-primary": "#181a1b",
        "darkness-secondary": "#393e41",
        "darkness-disabled": "#393e41",
      },
    },
  },
  variants: {},
  plugins: [],
  safelist: ["bg-slate-200", "bg-cyan-500"],
};
