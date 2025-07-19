import React from "react";
import { Sun, Moon } from "lucide-react";

export const ThemeToggle: React.FC = () => {
  const [dark, setDark] = React.useState(false);

  React.useEffect(() => {
    document.documentElement.classList.toggle("dark", dark);
  }, [dark]);

  return (
    <button
      className="rounded-full p-2 hover:bg-accent transition"
      onClick={() => setDark((d) => !d)}
      aria-label="Toggle theme"
    >
      {dark ? <Sun className="w-5 h-5 text-yellow-300" /> : <Moon className="w-5 h-5 text-zinc-600" />}
    </button>
  );
};
