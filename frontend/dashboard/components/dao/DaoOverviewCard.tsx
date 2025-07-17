import React from "react";
import { Users, Coins, FileText, BarChart2 } from "lucide-react";

interface DaoOverviewCardProps {
  daoName: string;
  avatarUrl?: string;
  members: number;
  treasury: string; // Formatted string, e.g. "₿ 1,245.50"
  proposals: number;
  activeProposals: number;
  votesToday: number;
  onClick?: () => void;
}

export const DaoOverviewCard: React.FC<DaoOverviewCardProps> = ({
  daoName,
  avatarUrl,
  members,
  treasury,
  proposals,
  activeProposals,
  votesToday,
  onClick,
}) => (
  <div
    className="bg-card rounded-2xl shadow-md border p-6 flex flex-col gap-3 hover:shadow-lg transition cursor-pointer group"
    onClick={onClick}
    tabIndex={0}
    aria-label={`Overview for ${daoName}`}
  >
    <div className="flex items-center gap-3 mb-2">
      {avatarUrl ? (
        <img
          src={avatarUrl}
          alt={`${daoName} avatar`}
          className="h-12 w-12 rounded-full object-cover border"
        />
      ) : (
        <div className="h-12 w-12 flex items-center justify-center bg-primary/10 rounded-full text-2xl font-bold text-primary select-none">
          {daoName[0]}
        </div>
      )}
      <span className="font-extrabold text-xl truncate">{daoName}</span>
    </div>

    <div className="grid grid-cols-2 sm:grid-cols-4 gap-3 mt-3">
      <div className="flex items-center gap-2">
        <Users className="w-5 h-5 text-accent" />
        <span className="font-semibold">{members}</span>
        <span className="text-xs text-muted-foreground">Members</span>
      </div>
      <div className="flex items-center gap-2">
        <Coins className="w-5 h-5 text-accent" />
        <span className="font-semibold">{treasury}</span>
        <span className="text-xs text-muted-foreground">Treasury</span>
      </div>
      <div className="flex items-center gap-2">
        <FileText className="w-5 h-5 text-accent" />
        <span className="font-semibold">{proposals}</span>
        <span className="text-xs text-muted-foreground">Proposals</span>
      </div>
      <div className="flex items-center gap-2">
        <BarChart2 className="w-5 h-5 text-accent" />
        <span className="font-semibold">{votesToday}</span>
        <span className="text-xs text-muted-foreground">Votes Today</span>
      </div>
    </div>

    <div className="flex items-center justify-between mt-4">
      <span className="text-xs text-muted-foreground">
        Active Proposals: <span className="font-semibold text-primary">{activeProposals}</span>
      </span>
      <button
        className="rounded-lg bg-primary text-white text-xs font-semibold px-4 py-1.5 shadow hover:bg-primary/90 focus:outline-none"
        onClick={onClick}
      >
        View DAO
      </button>
    </div>
  </div>
);
