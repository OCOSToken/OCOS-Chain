import React from "react";
import { User, CalendarDays, ChevronRight } from "lucide-react";

export interface ProposalCardProps {
  id?: string | number;
  title: string;
  proposer: string;
  createdAt: string;
  status: string;
  votesFor: number;
  votesAgainst: number;
  onClick?: () => void;
}

const statusStyles: Record<string, string> = {
  Pending: "bg-yellow-100 text-yellow-800",
  Approved: "bg-green-100 text-green-800",
  Rejected: "bg-red-100 text-red-800",
  Executed: "bg-blue-100 text-blue-800",
  Expired: "bg-gray-200 text-gray-700",
  Failed: "bg-gray-100 text-gray-500",
};

export const ProposalCard: React.FC<ProposalCardProps> = ({
  title,
  proposer,
  createdAt,
  status,
  votesFor,
  votesAgainst,
  onClick,
}) => (
  <div
    className="group bg-card rounded-2xl shadow-md border p-5 flex flex-col gap-2 hover:shadow-lg transition cursor-pointer focus:outline-none focus:ring-2 focus:ring-primary"
    onClick={onClick}
    tabIndex={0}
    aria-label={`Proposal: ${title}`}
    role="button"
    onKeyDown={(e) => (e.key === "Enter" || e.key === " ") && onClick?.()}
  >
    <div className="flex items-center justify-between">
      <h3 className="font-bold text-lg truncate">{title}</h3>
      <span
        className={`text-xs font-medium px-2 py-0.5 rounded-full ${statusStyles[status] || "bg-muted text-foreground"}`}
      >
        {status}
      </span>
    </div>
    <div className="flex items-center gap-4 text-xs text-muted-foreground mb-1">
      <span className="inline-flex items-center gap-1">
        <User className="w-4 h-4" /> {proposer}
      </span>
      <span className="inline-flex items-center gap-1">
        <CalendarDays className="w-4 h-4" /> {createdAt}
      </span>
    </div>
    <div className="flex gap-6 mt-1 mb-2">
      <span className="text-green-700 font-semibold text-sm">
        +{votesFor} <span className="text-xs">For</span>
      </span>
      <span className="text-red-600 font-semibold text-sm">
        -{votesAgainst} <span className="text-xs">Against</span>
      </span>
    </div>
    <div className="flex items-center justify-end">
      <span className="text-primary text-xs font-semibold group-hover:underline">
        View Details
      </span>
      <ChevronRight className="ml-1 w-4 h-4 text-primary group-hover:translate-x-1 transition-transform" />
    </div>
  </div>
);
