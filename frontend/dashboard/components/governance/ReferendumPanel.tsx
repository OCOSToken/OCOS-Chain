import React from "react";
import { BarChart2, Vote, CheckCircle, XCircle } from "lucide-react";
import { Button } from "@/components/shared/Button";

export interface ReferendumPanelProps {
  title: string;
  description: string;
  options: string[];
  votes: number[];
  status: string;
  onVote: (optionIdx: number) => void;
  isVotingOpen: boolean;
}

export const ReferendumPanel: React.FC<ReferendumPanelProps> = ({
  title,
  description,
  options,
  votes,
  status,
  onVote,
  isVotingOpen,
}) => {
  const totalVotes = votes.reduce((sum, v) => sum + v, 0);

  return (
    <section className="bg-card border rounded-2xl shadow p-6 w-full max-w-lg mx-auto mb-6">
      <div className="flex items-center gap-2 mb-3">
        <BarChart2 className="w-5 h-5 text-accent" />
        <h3 className="font-bold text-lg">{title}</h3>
        <span className={`ml-auto text-xs font-semibold ${status === "Passed" ? "text-green-600" : "text-red-600"}`}>
          {status === "Passed" ? <CheckCircle className="w-4 h-4" /> : <XCircle className="w-4 h-4" />}
          {status}
        </span>
      </div>
      <p className="mb-4 text-sm text-muted-foreground">{description}</p>
      <div className="grid gap-3">
        {options.map((option, idx) => (
          <Button
            key={option}
            className="bg-primary text-white font-semibold"
            onClick={() => onVote(idx)}
            disabled={!isVotingOpen}
          >
            {option} ({votes[idx] || 0} votes)
          </Button>
        ))}
      </div>
      <div className="mt-4 text-xs text-muted-foreground">
        Total votes: <b>{totalVotes}</b>
      </div>
    </section>
  );
};
