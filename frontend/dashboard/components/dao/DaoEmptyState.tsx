import React from "react";
import { FileText, Users, Search } from "lucide-react";

export interface DaoEmptyStateProps {
  type?: "proposal" | "member" | "vote" | "search" | string;
  message?: string;
  ctaLabel?: string;
  onCta?: () => void;
}

const icons: Record<string, React.ReactNode> = {
  proposal: <FileText className="w-10 h-10 text-accent mx-auto" />,
  member: <Users className="w-10 h-10 text-accent mx-auto" />,
  vote: <Search className="w-10 h-10 text-accent mx-auto" />,
  search: <Search className="w-10 h-10 text-accent mx-auto" />,
};

export const DaoEmptyState: React.FC<DaoEmptyStateProps> = ({
  type = "proposal",
  message,
  ctaLabel,
  onCta,
}) => (
  <div className="flex flex-col items-center justify-center min-h-[240px] text-center bg-card border rounded-2xl shadow px-8 py-12">
    <div className="mb-4">{icons[type] || <FileText className="w-10 h-10 text-accent mx-auto" />}</div>
    <div className="font-semibold text-lg mb-2">
      {message ||
        (type === "proposal"
          ? "No proposals yet."
          : type === "member"
          ? "No members yet."
          : "Nothing found.")}
    </div>
    {ctaLabel && onCta && (
      <button
        className="mt-4 bg-primary text-white font-semibold rounded-lg px-4 py-2 hover:bg-primary/90 transition"
        onClick={onCta}
      >
        {ctaLabel}
      </button>
    )}
  </div>
);
