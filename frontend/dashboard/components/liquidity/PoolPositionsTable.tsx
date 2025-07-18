import React, { useMemo, useState } from "react";
import { Coins, TrendingUp, Gift, User, ChevronDown, ChevronUp } from "lucide-react";
import { Button } from "@/components/shared/Button";

export interface PoolPosition {
  id: string | number;
  poolName: string;
  tokens: { symbol: string; iconUrl?: string }[];
  lpTokens: number;
  userShare: number; // percent
  entryDate: string; // ISO date
  currentValue: number;
  earnedRewards: number;
  status?: string;
}

interface PoolPositionsTableProps {
  positions: PoolPosition[];
  onWithdraw?: (position: PoolPosition) => void;
  onView?: (position: PoolPosition) => void;
}

export const PoolPositionsTable: React.FC<PoolPositionsTableProps> = ({
  positions,
  onWithdraw,
  onView,
}) => {
  const [sortKey, setSortKey] = useState<"poolName" | "currentValue" | "earnedRewards" | "userShare">("currentValue");
  const [sortAsc, setSortAsc] = useState(false);

  const sortedPositions = useMemo(() => {
    return [...positions].sort((a, b) => {
      const valA = a[sortKey] ?? 0;
      const valB = b[sortKey] ?? 0;
      if (typeof valA === "number" && typeof valB === "number") {
        return sortAsc ? valA - valB : valB - valA;
      }
      return sortAsc
        ? String(valA).localeCompare(String(valB))
        : String(valB).localeCompare(String(valA));
    });
  }, [positions, sortKey, sortAsc]);

  return (
    <section className="bg-card border shadow rounded-2xl p-6 w-full overflow-x-auto">
      <div className="flex items-center justify-between mb-4">
        <h3 className="font-bold text-lg flex items-center gap-2">
          <User className="w-5 h-5 text-accent" /> My Pool Positions
        </h3>
      </div>
      <table className="min-w-full text-sm">
        <thead>
          <tr className="border-b">
            <Th label="Pool" sortKey="poolName" {...{ sortKey, setSortKey, sortAsc, setSortAsc }} />
            <th className="px-3 py-2 text-left">Tokens</th>
            <th className="px-3 py-2 text-right">LP Tokens</th>
            <Th label="Share" sortKey="userShare" {...{ sortKey, setSortKey, sortAsc, setSortAsc }} />
            <th className="px-3 py-2 text-right">Entry</th>
            <Th label="Value" sortKey="currentValue" {...{ sortKey, setSortKey, sortAsc, setSortAsc }} />
            <Th label="Rewards" sortKey="earnedRewards" {...{ sortKey, setSortKey, sortAsc, setSortAsc }} />
            <th className="px-2 py-1 text-right">Actions</th>
          </tr>
        </thead>
        <tbody>
          {sortedPositions.length === 0 ? (
            <tr>
              <td colSpan={8} className="py-6 text-center text-muted-foreground">
                No LP positions found.
              </td>
            </tr>
          ) : (
            sortedPositions.map((pos) => (
              <tr key={pos.id} className="border-b hover:bg-muted/50 transition">
                <td className="px-3 py-2 font-medium">{pos.poolName}</td>
                <td className="px-3 py-2 flex gap-1">
                  {pos.tokens.map((t) =>
                    t.iconUrl ? (
                      <img
                        key={t.symbol}
                        src={t.iconUrl}
                        alt={t.symbol}
                        className="w-5 h-5 rounded-full border"
                        title={t.symbol}
                      />
                    ) : (
                      <Coins className="w-4 h-4 text-accent" key={t.symbol} />
                    )
                  )}
                </td>
                <td className="px-3 py-2 text-right font-mono">{pos.lpTokens}</td>
                <td className="px-3 py-2 text-right">{pos.userShare}%</td>
                <td className="px-3 py-2 text-right">{pos.entryDate.slice(0, 10)}</td>
                <td className="px-3 py-2 text-right font-semibold text-foreground">
                  ${pos.currentValue.toLocaleString()}
                </td>
                <td className="px-3 py-2 text-right text-green-700">
                  +{pos.earnedRewards}
                </td>
                <td className="px-2 py-2 text-right">
                  {onWithdraw && (
                    <Button
                      size="xs"
                      variant="destructive"
                      onClick={() => onWithdraw(pos)}
                      className="text-xs"
                    >
                      Withdraw
                    </Button>
                  )}
                  {onView && (
                    <Button
                      size="xs"
                      variant="outline"
                      onClick={() => onView(pos)}
                      className="text-xs ml-1"
                    >
                      View
                    </Button>
                  )}
                </td>
              </tr>
            ))
          )}
        </tbody>
      </table>
    </section>
  );
};

interface ThProps {
  label: string;
  sortKey: string;
  setSortKey: (k: any) => void;
  sortAsc: boolean;
  setSortAsc: (b: boolean) => void;
}
const Th: React.FC<ThProps> = ({ label, sortKey, setSortKey, sortAsc, setSortAsc }) => (
  <th
    className="px-3 py-2 cursor-pointer select-none text-left text-muted-foreground"
    onClick={() => {
      setSortAsc(sortKey === label.toLowerCase() ? !sortAsc : false);
      setSortKey(label.toLowerCase());
    }}
    scope="col"
  >
    <div className="flex items-center gap-1">
      {label}
      {sortKey === label.toLowerCase() &&
        (sortAsc ? (
          <ChevronUp className="w-3 h-3" />
        ) : (
          <ChevronDown className="w-3 h-3" />
        ))}
    </div>
  </th>
);
