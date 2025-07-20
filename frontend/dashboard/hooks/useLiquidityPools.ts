import { useEffect, useState, useCallback } from "react";

// Example liquidity pool type (customize fields to your backend/chain)
export interface LiquidityPool {
  id: string;
  poolName: string;
  tokens: { symbol: string; iconUrl?: string }[];
  tvl: number;
  apy?: number;
  status?: string;
  metadata?: string;
  [key: string]: any; // For extensibility
}

export interface UseLiquidityPoolsOptions {
  page?: number;
  pageSize?: number;
  status?: string; // "Active", "Deprecated", etc.
  search?: string;
  autoRefreshInterval?: number; // ms
}

interface LiquidityPoolsResult {
  pools: LiquidityPool[];
  loading: boolean;
  error: string | null;
  total: number;
  page: number;
  pageSize: number;
  refetch: () => void;
  setPage: (page: number) => void;
}

/**
 * useLiquidityPools
 * Fetches liquidity pools, supports filtering, pagination, and auto-refresh.
 * Replace fetch URL with your API/GraphQL or on-chain source.
 */
export function useLiquidityPools(options: UseLiquidityPoolsOptions = {}): LiquidityPoolsResult {
  const {
    page = 1,
    pageSize = 12,
    status,
    search = "",
    autoRefreshInterval = 0,
  } = options;

  const [pools, setPools] = useState<LiquidityPool[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [currentPage, setCurrentPage] = useState(page);

  const fetchPools = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      // Example REST API endpoint
      let url = `/api/defi/pools?page=${currentPage}&pageSize=${pageSize}`;
      if (status) url += `&status=${encodeURIComponent(status)}`;
      if (search) url += `&search=${encodeURIComponent(search)}`;

      const res = await fetch(url);
      if (!res.ok) throw new Error("Failed to fetch pools");
      const data = await res.json();

      setPools(data.pools || []);
      setTotal(data.total || 0);
    } catch (err: any) {
      setError(err.message || "Error loading pools");
    }
    setLoading(false);
  }, [currentPage, pageSize, status, search]);

  // Fetch pools on mount or when filters/page change
  useEffect(() => {
    fetchPools();
    let interval: NodeJS.Timeout | undefined;
    if (autoRefreshInterval > 0) {
      interval = setInterval(fetchPools, autoRefreshInterval);
    }
    return () => {
      if (interval) clearInterval(interval);
    };
  }, [fetchPools, autoRefreshInterval]);

  return {
    pools,
    loading,
    error,
    total,
    page: currentPage,
    pageSize,
    refetch: fetchPools,
    setPage: setCurrentPage,
  };
}
