import React from "react";
import { cn } from "@/lib/utils";

export interface BadgeProps {
  label: string;
  color?: string; // Tailwind bg class, e.g. "bg-green-100"
}

export const Badge: React.FC<BadgeProps> = ({
  label,
  color = "bg-primary/10 text-primary",
}) => (
  <span className={cn("inline-block rounded-full px-3 py-1 text-xs font-bold", color)}>
    {label}
  </span>
);
