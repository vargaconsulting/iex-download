// Keep Material’s base styles intact
tailwind.config = {
  corePlugins: { preflight: false },
};
// tailwind.config.js
module.exports = {
  content: ["./docs/**/*.md", "./overrides/**/*.html"],
  // Treat either .dark class or MkDocs Material's slate scheme as "dark"
  darkMode: ["class", '[data-md-color-scheme="slate"]'],
  theme: { extend: {} },
  plugins: [],
};
