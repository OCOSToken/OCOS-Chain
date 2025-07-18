import React, { useState } from "react";
import { Info, ChevronDown } from "lucide-react";

export interface FeeBreakdown {
  label: string;
  percent: number;
  color?: string; // for bar/legend
  recipient?: string; // e.g. "LP Providers", "Protocol", "Treasury"
}

export interface FeeInfoProps {
  totalFeeBps: number; // e.g. 30 = 0.3%
  breakdown?: FeeBreakdown[];
  className?: string;
}

export const FeeInfo: React.FC<FeeInfoProps> = ({
  totalFeeBps,
  breakdown = [],
  className = "",
}) => {
  const [open, setOpen] = useState(false);
  const percent = (totalFeeBps / 100).toFixed(2);

  return (
    <div className={`inline-flex items-center gap-2 ${className}`}>
      <span className="font-semibold text-xs text-muted-foreground">
        Fee: <span className="text-foreground">{percent}%</span>
      </span>
      <button
        type="button"
        aria-label="Show fee breakdown"
        className="hover:bg-accent/30 rounded-full p-1 focus:outline-none"
        onClick={() => setOpen((v) => !v)}
      >
        <Info className="w-4 h-4 text-primary" />
        <ChevronDown className={`w-3 h-3 ml-0.5 transition-transform ${open ? "rotate-180" : ""}`} />
      </button>
      {open && (
        <div className="absolute z-50 mt-2 bg-card border rounded-xl shadow-lg p-4 w-64 text-xs text-left">
          <div className="mb-2 font-bold">
            Total Fee: <span className="text-primary">{percent}%</span>
          </div>
          {breakdown.length === 0 ? (
            <div className="text-muted-foreground">No breakdown available.</div>
          ) : (
            <ul className="flex flex-col gap-1">
              {breakdown.map((item) => (
                <li key={item.label} className="flex items-center gap-2">
                  <span
                    className="inline-block rounded-full w-2.5 h-2.5"
                    style={{
                      background: item.color || "#3b82f6",
                      minWidth: "0.65rem",
                    }}
                  ></span>
                  <span className="font-semibold">{item.label}:</span>
                  <span className="ml-auto">
                    {item.percent}%{" "}
                    {item.recipient && (
                      <span className="text-muted-foreground">({item.recipient})</span>
                    )}
                  </span>
                </li>
              ))}
            </ul>
          )}
        </div>
      )}
    </div>
  );
};
