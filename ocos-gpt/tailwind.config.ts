import type { Config } from "tailwindcss";

export default {
  content: ["./app/**/*.{ts,tsx}", "./components/**/*.{ts,tsx}"],
  theme: {
    extend: {
      colors: {
        bg: "#0b0f14",
        card: "#0f1720",
        muted: "#8aa0b2"
      }
    }
  },
  plugins: []
} satisfies Config;
