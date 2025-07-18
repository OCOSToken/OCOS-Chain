import React from "react";
import { Zap, PauseCircle, Star, CheckCircle, Clock, Slash } from "lucide-react";

export interface PoolStatusTagProps {
  status: "Active" | "Boosted" | "Deprecated" | "Incentivized" | "Paused" | "New" | string;
  className?: string;
}

const statusConfig: Record<
  string,
  { label: string; color: string; Icon: React.FC<any> }
> = {
  Active: {
    label: "Active",
    color: "bg-green-100 text-green-800 border border-green-300",
    Icon: CheckCircle,
  },
  Boosted: {
    label: "Boosted",
    color: "bg-yellow-100 text-yellow-800 border border-yellow-300",
    Icon: Star,
  },
  Deprecated: {
    label: "Deprecated",
    color: "bg-gray-200 text-gray-800 border border-gray-300",
    Icon: Slash,
  },
  Incentivized: {
    label: "Incentivized",
    color: "bg-blue-100 text-blue-700 border border-blue-300",
    Icon: Zap,
  },
  Paused: {
    label: "Paused",
    color: "bg-red-100 text-red-700 border border-red-300",
    Icon: PauseCircle,
  },
  New: {
    label: "New",
    color: "bg-primary/10 text-primary border border-primary/30",
    Icon: Clock,
  },
};

export const PoolStatusTag: React.FC<PoolStatusTagProps> = ({
  status,
  className = "",
}) => {
  const conf =
    statusConfig[status] || {
      label: status,
      color: "bg-muted text-foreground border border-muted",
      Icon: CheckCircle,
    };
  return (
    <span
      className={`inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs font-semibold ${conf.color} ${className}`}
      title={conf.label}
    >
      <conf.Icon className="w-3.5 h-3.5" />
      {conf.label}
    </span>
  );
};
