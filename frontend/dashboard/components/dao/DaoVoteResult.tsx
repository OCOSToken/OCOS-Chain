import React from "react";
import { BarChartHorizontal, Info, Users } from "lucide-react";

interface DaoVoteResultProps {
  votesFor: number;
  votesAgainst: number;
  votesAbstain?: number;
  totalEligible?: number;
  turnout?: number; // percent (0–100)
  quorum?: number;
  threshold?: number;
  status?: "pending" | "approved" | "rejected" | "executed" | "expired" | "failed";
}

export const DaoVoteResult: React.FC<DaoVoteResultProps> = ({
  votesFor,
  votesAgainst,
  votesAbstain,
  totalEligible,
  turnout,
  quorum,
  threshold,
  status,
}) => {
  const totalVotes =
    votesFor + votesAgainst + (votesAbstain !== undefined ? votesAbstain : 0);

  const forPct = totalVotes ? (votesFor / totalVotes) * 100 : 0;
  const againstPct = totalVotes ? (votesAgainst / totalVotes) * 100 : 0;
  const abstainPct =
    votesAbstain !== undefined && totalVotes
      ? (votesAbstain / totalVotes) * 100
      : 0;

  return (
    <section className="bg-card border rounded-2xl shadow p-6 w-full max-w-xl mx-auto mb-6">
      <div className="flex items-center gap-2 mb-4">
        <BarChartHorizontal className="w-5 h-5 text-accent" />
        <h3 className="font-bold text-lg">Voting Results</h3>
      </div>
      <div className="flex flex-col gap-2 mb-3">
        <VoteBar
          label="For"
          color="bg-green-500"
          value={votesFor}
          percent={forPct}
        />
        <VoteBar
          label="Against"
          color="bg-red-500"
          value={votesAgainst}
          percent={againstPct}
        />
        {votesAbstain !== undefined && (
          <VoteBar
            label="Abstain"
            color="bg-gray-400"
            value={votesAbstain}
            percent={abstainPct}
          />
        )}
      </div>
      <div className="flex flex-wrap gap-x-6 gap-y-2 text-xs text-muted-foreground mt-3">
        <span className="flex items-center gap-1">
          <Users className="w-4 h-4" /> Turnout:{" "}
          <b>{turnout !== undefined ? turnout + "%" : totalVotes}</b>
        </span>
        {totalEligible !== undefined && (
          <span>
            Eligible: <b>{totalEligible}</b>
          </span>
        )}
        {quorum !== undefined && (
          <span>
            Quorum: <b>{quorum}</b>
          </span>
        )}
        {threshold !== undefined && (
          <span>
            Threshold: <b>{threshold}</b>
          </span>
        )}
        {status && (
          <span>
            Status: <b className="capitalize">{status}</b>
          </span>
        )}
      </div>
      {quorum !== undefined && totalVotes < quorum && (
        <div className="mt-4 text-yellow-700 flex items-center gap-2">
          <Info className="w-4 h-4" /> Quorum not met. Proposal may not be valid.
        </div>
      )}
    </section>
  );
};

interface VoteBarProps {
  label: string;
  color: string;
  value: number;
  percent: number;
}
const VoteBar: React.FC<VoteBarProps> = ({ label, color, value, percent }) => (
  <div>
    <div className="flex justify-between items-center mb-1 text-sm">
      <span className="font-semibold">{label}</span>
      <span className="font-mono">
        {value} <span className="ml-2 text-muted-foreground">{percent.toFixed(1)}%</span>
      </span>
    </div>
    <div className="h-3 w-full rounded-full bg-muted relative overflow-hidden">
      <div
        className={`absolute top-0 left-0 h-full rounded-full transition-all duration-500 ${color}`}
        style={{ width: `${percent}%` }}
      />
    </div>
  </div>
);
