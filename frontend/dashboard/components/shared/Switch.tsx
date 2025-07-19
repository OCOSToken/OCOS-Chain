import React from "react";
import { cn } from "@/lib/utils";

export interface SwitchProps {
  checked: boolean;
  onChange: (checked: boolean) => void;
  label?: string;
}

export const Switch: React.FC<SwitchProps> = ({ checked, onChange, label }) => (
  <div className="flex items-center gap-2">
    <button
      role="switch"
      aria-checked={checked}
      className={cn(
        "relative inline-block w-10 h-6 transition rounded-full",
        checked ? "bg-primary" : "bg-gray-300"
      )}
      onClick={() => onChange(!checked)}
      type="button"
    >
      <span
        className={cn(
          "absolute left-1 top-1 bg-white w-4 h-4 rounded-full shadow transition-transform",
          checked ? "translate-x-4" : ""
        )}
      />
    </button>
    {label && <span className="text-sm">{label}</span>}
  </div>
);
