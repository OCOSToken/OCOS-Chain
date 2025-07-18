import React from "react";

export interface PoolToken {
  symbol: string;
  iconUrl?: string;
}

export interface PoolTokensBadgeProps {
  tokens: PoolToken[];
  size?: "sm" | "md" | "lg";
  className?: string;
}

const sizeMap = {
  sm: "w-5 h-5 text-xs",
  md: "w-7 h-7 text-base",
  lg: "w-10 h-10 text-lg",
};

export const PoolTokensBadge: React.FC<PoolTokensBadgeProps> = ({
  tokens,
  size = "md",
  className = "",
}) => (
  <span className={`inline-flex items-center gap-[-0.7rem] ${className}`}>
    {tokens.map((t, idx) => (
      <span
        key={t.symbol}
        className={`relative z-${10 - idx} border-2 border-card rounded-full bg-background shadow`}
        style={{ marginLeft: idx ? "-0.7rem" : 0 }}
        title={t.symbol}
      >
        {t.iconUrl ? (
          <img
            src={t.iconUrl}
            alt={t.symbol}
            className={`${sizeMap[size]} rounded-full object-cover`}
          />
        ) : (
          <span
            className={`flex items-center justify-center bg-muted ${sizeMap[size]} rounded-full font-bold`}
          >
            {t.symbol[0]}
          </span>
        )}
      </span>
    ))}
    <span className="ml-2 font-semibold">{tokens.map((t) => t.symbol).join(" / ")}</span>
  </span>
);
