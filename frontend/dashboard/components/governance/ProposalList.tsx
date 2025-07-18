import React, { useState, useMemo } from "react";
import { ProposalCard } from "./ProposalCard";
import { Input } from "@/components/shared/Input";

export interface ProposalListItem {
  id: string | number;
  title: string;
  proposer: string;
  createdAt: string;
  status: string;
  votesFor: number;
  votesAgainst: number;
  onClick?: () => void;
}

interface ProposalListProps {
  proposals: ProposalListItem[];
  onProposalSelect?: (proposal: ProposalListItem) => void;
}

export const ProposalList: React.FC<ProposalListProps> = ({
  proposals,
  onProposalSelect,
}) => {
  const [query, setQuery] = useState("");
  const filtered = useMemo(() => {
    const q = query.trim().toLowerCase();
    if (!q) return proposals;
    return proposals.filter(
      (p) =>
        p.title.toLowerCase().includes(q) ||
        p.proposer.toLowerCase().includes(q)
    );
  }, [proposals, query]);

  return (
    <section>
      <div className="mb-6 flex items-center justify-between">
        <h2 className="text-2xl font-bold">Proposals</h2>
        <Input
          placeholder="Search proposals..."
          className="max-w-xs"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          aria-label="Search proposals"
        />
      </div>
      {filtered.length === 0 ? (
        <div className="text-center py-12 text-muted-foreground text-lg">
          No proposals found.
        </div>
      ) : (
        <div className="grid grid-cols-1 sm:grid-cols-2 gap-6">
          {filtered.map((proposal) => (
            <ProposalCard
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
