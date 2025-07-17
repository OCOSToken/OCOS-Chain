import React from "react";
import { User, CalendarDays, FileText, CheckCircle, XCircle, Clock, Zap, ShieldCheck } from "lucide-react";
import { ProposalStatus } from "@/types/dao";
import { Button } from "@/components/shared/Button"; // Use your project's Button component

export interface DaoProposalDetailsProps {
  title: string;
  proposer: string;
  createdAt: string;
  status: ProposalStatus;
  votesFor: number;
  votesAgainst: number;
  votesAbstain?: number;
  quorum?: number;
  threshold?: number;
  timeline?: { label: string; date: string; icon?: React.ReactNode }[];
  description: string;
  actions?: React.ReactNode; // For Vote/Execute/Withdraw/etc. buttons
  onVoteFor?: () => void;
  onVoteAgainst?: () => void;
  onAbstain?: () => void;
  isVotingOpen?: boolean;
  isExecutable?: boolean;
  onExecute?: () => void;
}

export const DaoProposalDetails: React.FC<DaoProposalDetailsProps> = ({
  title,
  proposer,
  createdAt,
  status,
  votesFor,
  votesAgainst,
  votesAbstain,
  quorum,
  threshold,
  timeline = [],
  description,
  actions,
  onVoteFor,
  onVoteAgainst,
  onAbstain,
  isVotingOpen = false,
  isExecutable = false,
  onExecute,
}) => {
  const statusColor: Record<ProposalStatus, string> = {
    Pending: "text-yellow-700",
    Approved: "text-green-700",
    Rejected: "text-red-700",
    Executed: "text-blue-700",
    Expired: "text-gray-400",
    Failed: "text-gray-400",
  };

  const statusIcon: Record<ProposalStatus, React.ReactNode> = {
    Pending: <Clock className="w-5 h-5 text-yellow-500" />,
    Approved: <CheckCircle className="w-5 h-5 text-green-600" />,
    Rejected: <XCircle className="w-5 h-5 text-red-600" />,
    Executed: <ShieldCheck className="w-5 h-5 text-blue-600" />,
    Expired: <Clock className="w-5 h-5 text-gray-400" />,
    Failed: <XCircle className="w-5 h-5 text-gray-400" />,
  };

  return (
    <section className="bg-card border shadow rounded-2xl p-8 w-full max-w-3xl mx-auto">
      <div className="flex items-center gap-4 mb-4">
        {statusIcon[status]}
        <h1 className="font-extrabold text-2xl flex-1">{title}</h1>
        <span className={`text-xs font-semibold px-3 py-1 rounded-full border ${statusColor[status]} border-current`}>
          {status}
        </span>
      </div>

      <div className="flex flex-col sm:flex-row gap-4 mb-6 text-xs text-muted-foreground">
        <span className="flex items-center gap-1">
          <User className="w-4 h-4" /> Proposer: <span className="font-semibold">{proposer}</span>
        </span>
        <span className="flex items-center gap-1">
          <CalendarDays className="w-4 h-4" /> Created: {createdAt}
        </span>
        {quorum && (
          <span className="flex items-center gap-1">
            <Zap className="w-4 h-4" /> Quorum: {quorum}
          </span>
        )}
        {threshold && (
          <span className="flex items-center gap-1">
            <FileText className="w-4 h-4" /> Threshold: {threshold}
          </span>
        )}
      </div>

      {timeline && timeline.length > 0 && (
        <div className="mb-6">
          <h3 className="font-bold text-base mb-1">Timeline</h3>
          <ol className="relative border-l-2 border-primary/40">
            {timeline.map((item, idx) => (
              <li key={idx} className="mb-3 ml-3">
                <div className="flex items-center gap-2">
                  <span className="w-4 h-4">{item.icon ?? <CircleDot />}</span>
                  <span className="font-medium">{item.label}</span>
                  <span className="ml-2 text-xs text-muted-foreground">{item.date}</span>
                </div>
              </li>
            ))}
          </ol>
        </div>
      )}

      <div className="mb-8">
        <h3 className="font-bold text-base mb-2">Description</h3>
        <p className="text-foreground whitespace-pre-line">{description}</p>
      </div>

      <div className="flex items-center gap-4 mb-4">
        <span className="flex items-center gap-1 text-green-700 font-bold text-sm">
          +{votesFor} For
        </span>
        <span className="flex items-center gap-1 text-red-600 font-bold text-sm">
          -{votesAgainst} Against
        </span>
        {votesAbstain !== undefined && (
          <span className="flex items-center gap-1 text-gray-600 font-bold text-sm">
            {votesAbstain} Abstain
          </span>
        )}
      </div>

      {actions ? (
        <div className="flex gap-3">{actions}</div>
      ) : (
        <div className="flex gap-3">
          {isVotingOpen && (
            <>
              <Button variant="primary" onClick={onVoteFor}>
                Vote For
              </Button>
              <Button variant="destructive" onClick={onVoteAgainst}>
                Vote Against
              </Button>
              <Button variant="outline" onClick={onAbstain}>
                Abstain
              </Button>
            </>
          )}
          {isExecutable && (
            <Button variant="success" onClick={onExecute}>
              Execute Proposal
            </Button>
          )}
        </div>
      )}
    </section>
  );
};

// Optional: CircleDot icon for timeline
const CircleDot = () => (
  <svg className="w-4 h-4 text-primary" fill="currentColor" viewBox="0 0 20 20">
    <circle cx="10" cy="10" r="3" />
  </svg>
);
