/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "jit",
  content: [
    "./src/**/*.rs",
    "./index.html",
    "./src/**/*.html",
    "./src/**/*.css",
  ],
  theme: {},
  variants: {},
  plugins: [],
  safelist: ["bg-slate-200", "bg-cyan-500"],
};
