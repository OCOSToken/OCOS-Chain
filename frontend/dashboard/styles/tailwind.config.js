/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "class", // Use "class" strategy for context/provider dark mode
  content: [
    "./pages/**/*.{js,ts,jsx,tsx}",
    "./components/**/*.{js,ts,jsx,tsx}",
    "./layout/**/*.{js,ts,jsx,tsx}",
    "./hooks/**/*.{js,ts,jsx,tsx}",
    "./services/**/*.{js,ts,jsx,tsx}",
    "./context/**/*.{js,ts,jsx,tsx}",
    "./app/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: "var(--color-primary)",
        accent: "var(--color-accent)",
        background: "var(--color-background)",
        card: "var(--color-card)",
        foreground: "var(--color-foreground)",
        muted: "var(--color-muted)",
        danger: "var(--color-danger)",
      },
      fontFamily: {
        sans: ["Inter", "Roboto", "Arial", "sans-serif"],
      },
      borderRadius: {
        xl: "var(--radius)",
        "2xl": "calc(var(--radius) * 1.5)",
        "3xl": "calc(var(--radius) * 2)",
      },
      boxShadow: {
        card: "0 4px 32px 0 rgba(25, 30, 58, 0.09)",
        "card-lg": "0 6px 40px 0 rgba(25, 30, 58, 0.14)",
        input: "0 2px 8px 0 rgba(25, 30, 58, 0.06)",
      },
      transitionProperty: {
        spacing: "margin, padding",
      },
      maxWidth: {
        "7xl": "90rem",
      },
    },
  },
  plugins: [
    require("@tailwindcss/forms"),     // Professional form styling
    require("@tailwindcss/typography"), // Prose/markdown support
    require("@tailwindcss/line-clamp"), // Multi-line text truncation
    require("@tailwindcss/aspect-ratio"),
  ],
};
