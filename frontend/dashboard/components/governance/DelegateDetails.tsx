import React from "react";
import { Star, User } from "lucide-react";

export interface DelegateDetailsProps {
  displayName: string;
  address: string;
  votingPower: number;
  proposalsVoted: number;
  recentVotes?: { proposal: string; date: string; vote: string }[];
}

export const DelegateDetails: React.FC<DelegateDetailsProps> = ({
  displayName,
  address,
  votingPower,
  proposalsVoted,
  recentVotes = [],
}) => (
  <section className="bg-card border rounded-2xl shadow p-8 w-full max-w-lg mx-auto mb-8">
    <div className="flex items-center gap-3 mb-2">
      <User className="w-7 h-7 text-accent" />
      <h2 className="font-bold text-2xl">{displayName}</h2>
      <Star className="w-5 h-5 text-yellow-500 ml-1" />
    </div>
    <div className="text-xs text-muted-foreground mb-2">
      Address: <span className="font-mono">{address}</span>
    </div>
    <div className="flex gap-4 mb-3 text-sm">
      <span>Voting Power: <b>{votingPower}</b></span>
      <span>Voted: <b>{proposalsVoted} proposals</b></span>
    </div>
    <div>
      <h4 className="font-semibold mb-1">Recent Votes</h4>
      {recentVotes.length === 0 ? (
        <div className="text-xs text-muted-foreground">No recent votes.</div>
      ) : (
        <ul className="flex flex-col gap-1">
          {recentVotes.map((v, idx) => (
            <li key={idx} className="flex gap-2 text-xs">
              <span className="font-semibold">{v.proposal}</span>
              <span className="text-muted-foreground">{v.date}</span>
              <span className="ml-2">{v.vote}</span>
            </li>
          ))}
        </ul>
      )}
    </div>
  </section>
);
