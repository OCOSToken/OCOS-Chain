import React from "react";
import { FileText, Plus, Vote, BarChartHorizontal, CheckCircle } from "lucide-react";
import { Button } from "@/components/shared/Button";
import { DaoProposalList, DaoVoteResult, DaoEmptyState } from "@/components/dao";

export interface GovernancePanelProps {
  proposals: {
    id: string | number;
    title: string;
    proposer: string;
    createdAt: string;
    status: string;
    votesFor: number;
    votesAgainst: number;
    onClick?: () => void;
  }[];
  onCreateProposal?: () => void;
  onProposalSelect?: (proposal: any) => void;
  votingStats?: {
    total: number;
    approved: number;
    rejected: number;
    turnout?: number;
  };
  canCreate?: boolean;
  className?: string;
}

export const GovernancePanel: React.FC<GovernancePanelProps> = ({
  proposals,
  onCreateProposal,
  onProposalSelect,
  votingStats,
  canCreate = false,
  className = "",
}) => (
  <section className={`bg-card border rounded-2xl shadow p-6 w-full max-w-3xl mx-auto ${className}`}>
    <div className="flex items-center gap-2 mb-4">
      <BarChartHorizontal className="w-5 h-5 text-accent" />
      <h2 className="font-bold text-xl flex-1">Governance</h2>
      {canCreate && onCreateProposal && (
        <Button
          className="bg-primary text-white font-semibold"
          onClick={onCreateProposal}
        >
          <Plus className="w-4 h-4 mr-1" /> New Proposal
        </Button>
      )}
    </div>
    {votingStats && (
      <div className="flex gap-6 text-xs text-muted-foreground mb-4">
        <span>
          <Vote className="w-4 h-4 inline mr-1 text-accent" /> {votingStats.total} proposals
        </span>
        <span>
          <CheckCircle className="w-4 h-4 inline mr-1 text-green-600" /> {votingStats.approved} approved
        </span>
        <span>
          <FileText className="w-4 h-4 inline mr-1 text-red-600" /> {votingStats.rejected} rejected
        </span>
        {votingStats.turnout !== undefined && (
          <span>
            Turnout: <b>{votingStats.turnout}%</b>
          </span>
        )}
      </div>
    )}
    {proposals.length === 0 ? (
      <DaoEmptyState
        type="proposal"
        message="No governance proposals yet."
        ctaLabel={canCreate ? "Create Proposal" : undefined}
        onCta={canCreate ? onCreateProposal : undefined}
      />
    ) : (
      <DaoProposalList proposals={proposals} onProposalSelect={onProposalSelect} />
    )}
  </section>
);
