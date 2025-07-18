import React from "react";
import { Globe, Clock, CheckCircle, AlertTriangle } from "lucide-react";

export interface OraclePricePanelProps {
  baseSymbol: string;
  quoteSymbol: string;
  price: number;
  source: string;
  lastUpdated: string; // ISO or formatted string
  confidence?: number; // % or decimal (optional)
  status?: "online" | "stale" | "error";
  className?: string;
}

export const OraclePricePanel: React.FC<OraclePricePanelProps> = ({
  baseSymbol,
  quoteSymbol,
  price,
  source,
  lastUpdated,
  confidence,
  status = "online",
  className = "",
}) => {
  const statusConfig = {
    online: {
      color: "text-green-600",
      label: "Online",
      icon: <CheckCircle className="w-4 h-4" />,
    },
    stale: {
      color: "text-yellow-600",
      label: "Stale",
      icon: <AlertTriangle className="w-4 h-4" />,
    },
    error: {
      color: "text-red-600",
      label: "Error",
      icon: <AlertTriangle className="w-4 h-4" />,
    },
  }[status];

  return (
    <section className={`bg-card border rounded-xl p-5 shadow w-full max-w-md ${className}`}>
      <div className="flex items-center gap-2 mb-2">
        <Globe className="w-5 h-5 text-accent" />
        <h3 className="font-bold text-base">
          Oracle Price: {baseSymbol}/{quoteSymbol}
        </h3>
        <span className={`ml-auto flex items-center gap-1 text-xs font-semibold ${statusConfig.color}`}>
          {statusConfig.icon}
          {statusConfig.label}
        </span>
      </div>
      <div className="flex items-baseline gap-2 mb-2">
        <span className="text-2xl font-bold text-primary">
          {price === 0 || isNaN(price) ? "-" : price.toLocaleString(undefined, { maximumFractionDigits: 8 })}
        </span>
        <span className="text-xs text-muted-foreground font-mono">
          {quoteSymbol}
        </span>
        {confidence !== undefined && (
          <span className="ml-3 text-xs text-muted-foreground">
            Confidence: <b>{confidence}%</b>
          </span>
        )}
      </div>
      <div className="flex items-center gap-2 text-xs text-muted-foreground">
        <span>Source: <b>{source}</b></span>
        <span className="mx-2">&bull;</span>
        <span className="flex items-center gap-1">
          <Clock className="w-3.5 h-3.5" /> Updated: {lastUpdated}
        </span>
      </div>
    </section>
  );
};
