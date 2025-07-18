import React from "react";
import { Gift, Coins, CheckCircle, Loader2 } from "lucide-react";
import { Button } from "@/components/shared/Button";

export interface PoolRewardToken {
  symbol: string;
  iconUrl?: string;
  pending: number;
  claimed: number;
  apr?: number;
}

export interface PoolRewardsPanelProps {
  tokens: PoolRewardToken[];
  lastClaimed?: string; // ISO date or formatted string
  isClaiming?: boolean;
  onClaim?: () => Promise<void> | void;
  totalApr?: number;
  poolName?: string;
}

export const PoolRewardsPanel: React.FC<PoolRewardsPanelProps> = ({
  tokens,
  lastClaimed,
  isClaiming = false,
  onClaim,
  totalApr,
  poolName,
}) => {
  const pendingTotal = tokens.reduce((sum, t) => sum + t.pending, 0);

  return (
    <section className="bg-card border rounded-2xl shadow p-6 w-full max-w-xl mx-auto mb-6">
      <div className="flex items-center gap-2 mb-4">
        <Gift className="w-5 h-5 text-accent" />
        <h3 className="font-bold text-lg">
          {poolName ? `${poolName} Rewards` : "Farming Rewards"}
        </h3>
        {totalApr !== undefined && (
          <span className="ml-3 bg-primary/10 text-primary px-2 py-0.5 rounded-full font-semibold text-xs">
            Total APR: {totalApr}%
          </span>
        )}
      </div>
      <div className="grid grid-cols-2 gap-4 mb-3">
        {tokens.length === 0 ? (
          <span className="col-span-2 text-center text-muted-foreground">
            No rewards earned yet.
          </span>
        ) : (
          tokens.map((t) => (
            <div key={t.symbol} className="flex flex-col gap-1 p-3 bg-muted rounded-lg">
              <div className="flex items-center gap-2">
                {t.iconUrl ? (
                  <img src={t.iconUrl} alt={t.symbol} className="w-6 h-6 rounded-full" />
                ) : (
                  <Coins className="w-5 h-5 text-accent" />
                )}
                <span className="font-semibold">{t.symbol}</span>
                {t.apr !== undefined && (
                  <span className="ml-auto text-xs text-muted-foreground">
                    APR: {t.apr}%
                  </span>
                )}
              </div>
              <div className="flex gap-3 mt-1 text-sm">
                <span>
                  Pending: <b className="text-foreground">{t.pending}</b>
                </span>
                <span>
                  Claimed: <b className="text-muted-foreground">{t.claimed}</b>
                </span>
              </div>
            </div>
          ))
        )}
      </div>
      <div className="flex items-center justify-between mt-3">
        <div className="text-xs text-muted-foreground">
          {lastClaimed && (
            <span>
              <CheckCircle className="w-4 h-4 inline mr-1 text-green-600" />
              Last claimed: {lastClaimed}
            </span>
          )}
        </div>
        {onClaim && (
          <Button
            className="bg-primary text-white font-semibold"
            onClick={onClaim}
            disabled={isClaiming || pendingTotal === 0}
          >
            {isClaiming ? (
              <>
                <Loader2 className="w-4 h-4 mr-1 animate-spin" /> Claiming...
              </>
            ) : (
              <>Claim All</>
            )}
          </Button>
        )}
      </div>
    </section>
  );
};
