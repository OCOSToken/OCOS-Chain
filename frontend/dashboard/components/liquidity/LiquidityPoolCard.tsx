import React from "react";
import { PoolTokensBadge } from "./PoolTokensBadge";
import { PoolStatusTag } from "./PoolStatusTag";
import { Coins } from "lucide-react";

export interface LiquidityPoolCardProps {
  id?: string | number;
  poolName: string;
  tokens: { symbol: string; iconUrl?: string }[];
  tvl: number;
  apy?: number;
  status?: string;
  onClick?: () => void;
}

export const LiquidityPoolCard: React.FC<LiquidityPoolCardProps> = ({
  poolName,
  tokens,
  tvl,
  apy,
  status,
  onClick,
}) => (
  <div
    className="bg-card border rounded-2xl p-5 shadow hover:shadow-lg transition cursor-pointer group"
    onClick={onClick}
    tabIndex={0}
    aria-label={`Liquidity Pool: ${poolName}`}
    role="button"
    onKeyDown={(e) => (e.key === "Enter" || e.key === " ") && onClick?.()}
  >
    <div className="flex items-center gap-3 mb-2">
      <PoolTokensBadge tokens={tokens} />
      <span className="font-bold text-lg flex-1 truncate">{poolName}</span>
      {status && <PoolStatusTag status={status} />}
    </div>
    <div className="flex gap-6 mt-2 text-sm text-muted-foreground">
      <span>
        TVL:{" "}
        <b className="text-foreground">${tvl.toLocaleString()}</b>
      </span>
      {apy !== undefined && (
        <span>
          APY: <b className="text-foreground">{apy}%</b>
        </span>
      )}
    </div>
    <div className="flex items-center gap-2 mt-3 text-xs text-accent font-medium">
      <Coins className="w-4 h-4" /> Click for details
    </div>
  </div>
);
