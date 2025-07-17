import React, { useState } from "react";
import { ThumbsUp, ThumbsDown, MinusCircle, Loader2 } from "lucide-react";
import { Button } from "@/components/shared/Button";

export interface DaoVotePanelProps {
  onVoteFor: (weight?: number) => Promise<void> | void;
  onVoteAgainst: (weight?: number) => Promise<void> | void;
  onAbstain?: (weight?: number) => Promise<void> | void;
  isVotingOpen: boolean;
  canVote: boolean;
  isWeighted?: boolean;
  minWeight?: number;
  maxWeight?: number;
  statusMsg?: string;
  voterWeight?: number;
}

export const DaoVotePanel: React.FC<DaoVotePanelProps> = ({
  onVoteFor,
  onVoteAgainst,
  onAbstain,
  isVotingOpen,
  canVote,
  isWeighted = false,
  minWeight = 1,
  maxWeight = 100,
  statusMsg,
  voterWeight = minWeight,
}) => {
  const [weight, setWeight] = useState<number>(voterWeight);
  const [pending, setPending] = useState<"for" | "against" | "abstain" | null>(null);

  const handleVote = async (
    type: "for" | "against" | "abstain",
    action: (w?: number) => Promise<void> | void
  ) => {
    setPending(type);
    try {
      await action(isWeighted ? weight : undefined);
    } finally {
      setPending(null);
    }
  };

  return (
    <div className="rounded-2xl bg-card border shadow px-6 py-5 flex flex-col gap-4 w-full max-w-md mx-auto">
      <h3 className="font-bold text-lg mb-1">Cast Your Vote</h3>

      {isWeighted && (
        <div className="mb-2 flex flex-col items-center">
          <label htmlFor="vote-weight" className="text-xs text-muted-foreground mb-1">
            Voting Weight
          </label>
          <input
            id="vote-weight"
            type="number"
            min={minWeight}
            max={maxWeight}
            value={weight}
            onChange={(e) => setWeight(Number(e.target.value))}
            className="w-28 border rounded px-2 py-1 text-center font-semibold text-base"
            disabled={!isVotingOpen || !canVote}
          />
          <span className="text-xs text-muted-foreground mt-1">
            Min: {minWeight}, Max: {maxWeight}
          </span>
        </div>
      )}

      <div className="flex justify-center gap-4 mt-2">
        <Button
          className="bg-green-600 hover:bg-green-700 text-white font-semibold flex items-center gap-2"
          disabled={!isVotingOpen || !canVote || pending !== null}
          onClick={() => handleVote("for", onVoteFor)}
          aria-label="Vote For"
        >
          {pending === "for" ? <Loader2 className="w-4 h-4 animate-spin" /> : <ThumbsUp className="w-5 h-5" />}
          For
        </Button>
        <Button
          className="bg-red-600 hover:bg-red-700 text-white font-semibold flex items-center gap-2"
          disabled={!isVotingOpen || !canVote || pending !== null}
          onClick={() => handleVote("against", onVoteAgainst)}
          aria-label="Vote Against"
        >
          {pending === "against" ? <Loader2 className="w-4 h-4 animate-spin" /> : <ThumbsDown className="w-5 h-5" />}
          Against
        </Button>
        {onAbstain && (
          <Button
            className="bg-gray-200 hover:bg-gray-300 text-gray-700 font-semibold flex items-center gap-2"
            disabled={!isVotingOpen || !canVote || pending !== null}
            onClick={() => handleVote("abstain", onAbstain)}
            aria-label="Abstain"
          >
            {pending === "abstain" ? <Loader2 className="w-4 h-4 animate-spin" /> : <MinusCircle className="w-5 h-5" />}
            Abstain
          </Button>
        )}
      </div>

      <div className="mt-4 text-xs text-muted-foreground text-center min-h-[1.5rem]">
        {!isVotingOpen && <span>Voting is closed.</span>}
        {isVotingOpen && !canVote && <span>You are not eligible to vote on this proposal.</span>}
        {statusMsg && <span>{statusMsg}</span>}
      </div>
    </div>
  );
};
