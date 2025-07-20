import { useEffect, useState } from "react";

export type ThemeMode = "light" | "dark";

interface UseDarkModeResult {
  isDark: boolean;
  mode: ThemeMode;
  toggle: () => void;
  setMode: (mode: ThemeMode) => void;
}

/**
 * useDarkMode
 * Detects and sets dark/light mode, persists to localStorage, and syncs with Tailwind/Next.js themes.
 */
export function useDarkMode(defaultMode: ThemeMode = "light"): UseDarkModeResult {
  const [mode, setModeState] = useState<ThemeMode>(() => {
    if (typeof window !== "undefined") {
      const stored = localStorage.getItem("theme_mode") as ThemeMode | null;
      if (stored === "dark" || stored === "light") return stored;
      // System preference as fallback
      if (window.matchMedia("(prefers-color-scheme: dark)").matches) return "dark";
    }
    return defaultMode;
  });

  useEffect(() => {
    document.documentElement.classList.toggle("dark", mode === "dark");
    localStorage.setItem("theme_mode", mode);
  }, [mode]);

  // Listen to system theme changes
  useEffect(() => {
    const media = window.matchMedia("(prefers-color-scheme: dark)");
    const handler = (e: MediaQueryListEvent) => {
      setModeState(e.matches ? "dark" : "light");
    };
    media.addEventListener("change", handler);
    return () => media.removeEventListener("change", handler);
  }, []);

  const setMode = (m: ThemeMode) => setModeState(m);
  const toggle = () => setModeState((m) => (m === "dark" ? "light" : "dark"));

  return {
    isDark: mode === "dark",
    mode,
    toggle,
    setMode,
  };
}
