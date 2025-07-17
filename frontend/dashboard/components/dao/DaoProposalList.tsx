import React, { useMemo, useState } from "react";
import { DaoProposalCard } from "./DaoProposalCard";
import { Input } from "@/components/shared/Input"; // Use your shared input component
import { ProposalStatus } from "@/types/dao";

export interface DaoProposalListItem {
  id: string | number;
  title: string;
  proposer: string;
  createdAt: string;
  status: ProposalStatus;
  votesFor: number;
  votesAgainst: number;
  onClick?: () => void;
}

interface DaoProposalListProps {
  proposals: DaoProposalListItem[];
  onProposalSelect?: (proposal: DaoProposalListItem) => void;
  showStatusFilter?: boolean;
}

export const DaoProposalList: React.FC<DaoProposalListProps> = ({
  proposals,
  onProposalSelect,
  showStatusFilter = true,
}) => {
  const [query, setQuery] = useState("");
  const [status, setStatus] = useState<ProposalStatus | "All">("All");

  // Filter and sort proposals
  const filteredProposals = useMemo(() => {
    let filtered = proposals;
    if (status !== "All") {
      filtered = filtered.filter((p) => p.status === status);
    }
    if (query.trim()) {
      const q = query.toLowerCase();
      filtered = filtered.filter(
        (p) =>
          p.title.toLowerCase().includes(q) ||
          p.proposer.toLowerCase().includes(q)
      );
    }
    // Optional: Sort by createdAt DESC
    return [...filtered].sort(
      (a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime()
    );
  }, [proposals, query, status]);

  const statusOptions: (ProposalStatus | "All")[] = [
    "All",
    "Pending",
    "Approved",
    "Rejected",
    "Executed",
    "Expired",
    "Failed",
  ];

  return (
    <section>
      <div className="mb-6 flex flex-col md:flex-row md:items-center md:justify-between gap-4">
        <h2 className="text-2xl font-bold">Proposals</h2>
        <div className="flex gap-2 items-center">
          {showStatusFilter && (
            <select
              className="border rounded-md px-3 py-1 bg-background text-sm text-foreground focus:outline-none"
              value={status}
              onChange={(e) => setStatus(e.target.value as ProposalStatus | "All")}
              aria-label="Filter by proposal status"
            >
              {statusOptions.map((opt) => (
                <option key={opt} value={opt}>
                  {opt}
                </option>
              ))}
            </select>
          )}
          <Input
            placeholder="Search proposals..."
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            className="max-w-xs"
            aria-label="Search proposals"
          />
        </div>
      </div>

      {filteredProposals.length === 0 ? (
        <div className="text-center py-12 text-muted-foreground text-lg">
          No proposals found for your search.
        </div>
      ) : (
        <div className="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 gap-6">
          {filteredProposals.map((proposal) => (
            <DaoProposalCard
              key={proposal.id}
              {...proposal}
              onClick={() => onProposalSelect?.(proposal)}
            />
          ))}
        </div>
      )}
    </section>
  );
};
