import React from "react";
import { PoolTokensBadge } from "./PoolTokensBadge";
import { PoolStatusTag } from "./PoolStatusTag";
import { PoolChart } from "./PoolChart"; // e.g., TVL/price/volume chart
import { PoolDepositForm } from "./PoolDepositForm";
import { PoolWithdrawForm } from "./PoolWithdrawForm";
import { PoolSwapForm } from "./PoolSwapForm";
import { PoolStatsPanel } from "./PoolStatsPanel";
import { PoolActivityFeed } from "./PoolActivityFeed";
import { Button } from "@/components/shared/Button";
import { Coins, TrendingUp, Swap, ChevronLeft } from "lucide-react";

export interface LiquidityPoolDetailsProps {
  id: string | number;
  poolName: string;
  tokens: { symbol: string; iconUrl?: string }[];
  tvl: number;
  apy?: number;
  status?: string;
  description?: string;
  chartData?: any; // Chart.js or recharts dataset, etc.
  onBack?: () => void;
}

export const LiquidityPoolDetails: React.FC<LiquidityPoolDetailsProps> = ({
  poolName,
  tokens,
  tvl,
  apy,
  status,
  description,
  chartData,
  onBack,
}) => {
  const [tab, setTab] = React.useState<"deposit" | "withdraw" | "swap">("deposit");

  return (
    <section className="bg-card border shadow rounded-2xl p-8 w-full max-w-3xl mx-auto mb-8">
      <div className="flex items-center gap-3 mb-3">
        {onBack && (
          <Button variant="ghost" size="icon" className="mr-1" onClick={onBack} aria-label="Back">
            <ChevronLeft className="w-5 h-5" />
          </Button>
        )}
        <PoolTokensBadge tokens={tokens} />
        <h1 className="font-extrabold text-2xl flex-1 truncate">{poolName}</h1>
        {status && <PoolStatusTag status={status} />}
      </div>
      <div className="flex flex-wrap gap-4 mt-1 mb-3 text-sm text-muted-foreground">
        <span>
          TVL: <b className="text-foreground">${tvl.toLocaleString()}</b>
        </span>
        {apy !== undefined && (
          <span>
            APY: <b className="text-foreground">{apy}%</b>
          </span>
        )}
      </div>
      {description && (
        <div className="mb-4 text-base text-muted-foreground">{description}</div>
      )}

      <div className="mb-6">
        <PoolStatsPanel /* props: stats, fees, rewards, etc. */ />
      </div>

      <div className="mb-8">
        {chartData && (
          <PoolChart data={chartData} /* type="tvl" or "volume" */ />
        )}
      </div>

      {/* Tabbed actions: Deposit | Withdraw | Swap */}
      <div className="flex gap-2 mb-4">
        <TabButton
          active={tab === "deposit"}
          onClick={() => setTab("deposit")}
          icon={<Coins className="w-4 h-4" />}
          label="Deposit"
        />
        <TabButton
          active={tab === "withdraw"}
          onClick={() => setTab("withdraw")}
          icon={<TrendingUp className="w-4 h-4" />}
          label="Withdraw"
        />
        <TabButton
          active={tab === "swap"}
          onClick={() => setTab("swap")}
          icon={<Swap className="w-4 h-4" />}
          label="Swap"
        />
      </div>

      <div className="mb-8">
        {tab === "deposit" && <PoolDepositForm /* poolId, tokens, ... */ />}
        {tab === "withdraw" && <PoolWithdrawForm /* poolId, tokens, ... */ />}
        {tab === "swap" && <PoolSwapForm /* poolId, tokens, ... */ />}
      </div>

      <div>
        <PoolActivityFeed /* poolId, events, ... */ />
      </div>
    </section>
  );
};

interface TabButtonProps {
  active: boolean;
  onClick: () => void;
  icon: React.ReactNode;
  label: string;
}
const TabButton: React.FC<TabButtonProps> = ({ active, onClick, icon, label }) => (
  <button
    className={`flex items-center gap-1 px-4 py-2 rounded-lg font-semibold text-sm ${
      active
        ? "bg-primary text-white shadow"
        : "bg-muted text-foreground hover:bg-accent"
    } transition`}
    onClick={onClick}
    type="button"
    aria-pressed={active}
  >
    {icon}
    {label}
  </button>
);
