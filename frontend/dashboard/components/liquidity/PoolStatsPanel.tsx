import React from "react";
import { Coins, Percent, TrendingUp, ArrowDownUp, Gift, User } from "lucide-react";

export interface PoolStatsPanelProps {
  tvl: number;
  apy?: number;
  volume24h?: number;
  fees24h?: number;
  rewardsPerDay?: number;
  userShare?: number; // percent
  poolStatus?: string; // e.g. "Active", "Boosted"
}

export const PoolStatsPanel: React.FC<PoolStatsPanelProps> = ({
  tvl,
  apy,
  volume24h,
  fees24h,
  rewardsPerDay,
  userShare,
  poolStatus,
}) => (
  <section className="grid grid-cols-2 md:grid-cols-3 gap-5 bg-muted rounded-xl p-5 w-full max-w-3xl mx-auto mb-4">
    <Stat
      label="TVL"
      value={`$${tvl.toLocaleString()}`}
      icon={<Coins className="w-5 h-5 text-accent" />}
    />
    <Stat
      label="APY"
      value={apy !== undefined ? `${apy}%` : "-"}
      icon={<Percent className="w-5 h-5 text-accent" />}
    />
    <Stat
      label="24h Volume"
      value={volume24h !== undefined ? `$${volume24h.toLocaleString()}` : "-"}
      icon={<TrendingUp className="w-5 h-5 text-accent" />}
    />
    <Stat
      label="24h Fees"
      value={fees24h !== undefined ? `$${fees24h.toLocaleString()}` : "-"}
      icon={<ArrowDownUp className="w-5 h-5 text-accent" />}
    />
    <Stat
      label="Rewards/Day"
      value={rewardsPerDay !== undefined ? `$${rewardsPerDay.toLocaleString()}` : "-"}
      icon={<Gift className="w-5 h-5 text-accent" />}
    />
    <Stat
      label="Your Share"
      value={userShare !== undefined ? `${userShare}%` : "-"}
      icon={<User className="w-5 h-5 text-accent" />}
    />
    {poolStatus && (
      <div className="col-span-2 md:col-span-3 text-center mt-2">
        <span className="inline-block bg-primary/10 text-primary px-3 py-1 rounded-full font-semibold text-xs">
          {poolStatus}
        </span>
      </div>
    )}
  </section>
);

interface StatProps {
  label: string;
  value: React.ReactNode;
  icon: React.ReactNode;
}
const Stat: React.FC<StatProps> = ({ label, value, icon }) => (
  <div className="flex flex-col items-center gap-1 p-2">
    <div>{icon}</div>
    <div className="font-semibold text-lg">{value}</div>
    <div className="text-xs text-muted-foreground">{label}</div>
  </div>
);
