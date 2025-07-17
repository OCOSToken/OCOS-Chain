import React from "react";
import { ShieldCheck, Star } from "lucide-react";

export interface DaoBadgeProps {
  daoName: string;
  avatarUrl?: string;
  color?: string; // Tailwind color (e.g., "bg-blue-600")
  label?: string;
  status?: "Verified" | "Featured" | "New" | string;
  size?: "sm" | "md" | "lg";
}

const sizeMap = {
  sm: "h-7 w-7 text-base px-2 py-0.5",
  md: "h-10 w-10 text-lg px-3 py-1",
  lg: "h-14 w-14 text-xl px-4 py-1.5",
};

const statusMap: Record<string, React.ReactNode> = {
  Verified: <ShieldCheck className="w-4 h-4 text-blue-500 ml-1" />,
  Featured: <Star className="w-4 h-4 text-yellow-500 ml-1" />,
  New: <span className="ml-1 text-xs font-bold text-green-500">New</span>,
};

export const DaoBadge: React.FC<DaoBadgeProps> = ({
  daoName,
  avatarUrl,
  color = "bg-primary/90",
  label,
  status,
  size = "md",
}) => {
  return (
    <span
      className={`inline-flex items-center gap-2 rounded-full font-semibold shadow-sm select-none ${sizeMap[size]} ${color} text-white`}
      title={daoName}
      role="img"
      aria-label={daoName + (status ? " " + status : "")}
    >
      {avatarUrl ? (
        <img
          src={avatarUrl}
          alt={`${daoName} avatar`}
          className="inline-block rounded-full object-cover border w-6 h-6"
        />
      ) : (
        <span className="flex items-center justify-center w-6 h-6 rounded-full bg-white/10 font-bold">
          {daoName[0]}
        </span>
      )}
      <span>{label || daoName}</span>
      {status && statusMap[status]}
    </span>
  );
};
