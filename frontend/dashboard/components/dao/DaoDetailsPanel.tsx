import React from "react";
import { Coins, Users, FileText, ShieldCheck, ExternalLink } from "lucide-react";
import { Button } from "@/components/shared/Button"; // Replace with your button component if different
import { DaoListItem } from "@/types/dao";

interface DaoDetailsPanelProps {
  dao: DaoListItem & {
    description?: string;
    website?: string;
    governanceType?: string; // e.g. "Council", "Weighted", "Quadratic", etc.
    creator?: string;
    socialLinks?: { label: string; url: string }[];
    isMember?: boolean;
  };
  onJoin?: () => void;
  onFollow?: () => void;
  onViewProposals?: () => void;
}

export const DaoDetailsPanel: React.FC<DaoDetailsPanelProps> = ({
  dao,
  onJoin,
  onFollow,
  onViewProposals,
}) => (
  <aside className="bg-card border shadow-md rounded-2xl p-8 w-full max-w-2xl mx-auto">
    <div className="flex items-center gap-4 mb-4">
      {dao.avatarUrl ? (
        <img
          src={dao.avatarUrl}
          alt={`${dao.daoName} avatar`}
          className="w-14 h-14 rounded-full border object-cover"
        />
      ) : (
        <div className="w-14 h-14 flex items-center justify-center bg-primary/10 rounded-full text-3xl font-bold text-primary">
          {dao.daoName[0]}
        </div>
      )}
      <div>
        <h1 className="text-2xl font-extrabold mb-1">{dao.daoName}</h1>
        {dao.governanceType && (
          <span className="inline-flex items-center gap-1 text-xs font-medium bg-accent rounded px-2 py-0.5">
            <ShieldCheck className="w-3 h-3 text-primary" /> {dao.governanceType} Governance
          </span>
        )}
      </div>
    </div>

    {dao.description && (
      <p className="text-muted-foreground mb-4">{dao.description}</p>
    )}

    <div className="grid grid-cols-2 sm:grid-cols-3 gap-4 mt-2 mb-6">
      <div className="flex items-center gap-2">
        <Users className="w-5 h-5 text-accent" />
        <span className="font-semibold">{dao.members}</span>
        <span className="text-xs text-muted-foreground">Members</span>
      </div>
      <div className="flex items-center gap-2">
        <Coins className="w-5 h-5 text-accent" />
        <span className="font-semibold">{dao.treasury}</span>
        <span className="text-xs text-muted-foreground">Treasury</span>
      </div>
      <div className="flex items-center gap-2">
        <FileText className="w-5 h-5 text-accent" />
        <span className="font-semibold">{dao.proposals}</span>
        <span className="text-xs text-muted-foreground">Proposals</span>
      </div>
    </div>

    <div className="flex flex-wrap gap-3 mt-2 mb-4">
      {dao.website && (
        <a
          href={dao.website}
          target="_blank"
          rel="noopener noreferrer"
          className="inline-flex items-center gap-1 text-sm text-primary underline hover:text-primary/80"
        >
          Website <ExternalLink className="w-4 h-4" />
        </a>
      )}
      {dao.socialLinks &&
        dao.socialLinks.map((link) => (
          <a
            key={link.url}
            href={link.url}
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex items-center gap-1 text-xs text-muted-foreground hover:underline"
          >
            {link.label} <ExternalLink className="w-3 h-3" />
          </a>
        ))}
    </div>

    <div className="flex gap-3 mt-4">
      {!dao.isMember && (
        <Button onClick={onJoin} className="bg-primary text-white font-semibold">
          Join DAO
        </Button>
      )}
      <Button onClick={onFollow} variant="outline">
        Follow
      </Button>
      <Button onClick={onViewProposals} variant="ghost">
        View Proposals
      </Button>
    </div>
  </aside>
);
