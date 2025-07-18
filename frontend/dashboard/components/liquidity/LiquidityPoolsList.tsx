import React, { useMemo, useState } from "react";
import { LiquidityPoolCard } from "./LiquidityPoolCard";
import { Input } from "@/components/shared/Input"; // Use your shared Input component

export interface PoolToken {
  symbol: string;
  iconUrl?: string;
}

export interface LiquidityPoolListItem {
  id: string | number;
  poolName: string;
  tokens: PoolToken[];
  tvl: number;
  apy?: number;
  status?: string;
  metadata?: string;
  onClick?: () => void;
}

interface LiquidityPoolsListProps {
  pools: LiquidityPoolListItem[];
  onPoolSelect?: (pool: LiquidityPoolListItem) => void;
}

export const LiquidityPoolsList: React.FC<LiquidityPoolsListProps> = ({
  pools,
  onPoolSelect,
}) => {
  const [query, setQuery] = useState("");

  const filteredPools = useMemo(() => {
    const q = query.trim().toLowerCase();
    if (!q) return pools;
    return pools.filter(
      (pool) =>
        pool.poolName.toLowerCase().includes(q) ||
        pool.tokens.some((t) => t.symbol.toLowerCase().includes(q)) ||
        (pool.metadata?.toLowerCase().includes(q) ?? false)
    );
  }, [pools, query]);

  return (
    <section>
      <div className="mb-6 flex items-center justify-between">
        <h2 className="text-2xl font-bold">Liquidity Pools</h2>
        <Input
          placeholder="Search pools..."
          className="max-w-xs"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          aria-label="Search pools"
        />
      </div>
      {filteredPools.length === 0 ? (
        <div className="text-center py-12 text-muted-foreground text-lg">
          No pools found for your search.
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
          {filteredPools.map((pool) => (
            <LiquidityPoolCard
              key={pool.id}
              {...pool}
              onClick={() => onPoolSelect?.(pool)}
            />
          ))}
        </div>
      )}
    </section>
  );
};
