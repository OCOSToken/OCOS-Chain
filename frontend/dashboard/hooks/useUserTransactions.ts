import { useEffect, useState, useCallback } from "react";

// Example transaction type (customize for your protocol/chain)
export interface UserTransaction {
  hash: string;
  date: string;           // ISO or block timestamp
  type: "IN" | "OUT" | "SWAP" | "STAKE" | "UNSTAKE" | "GOV" | string;
  amount: number;
  symbol: string;
  toFrom: string;
  explorerUrl?: string;
  status?: "Pending" | "Success" | "Failed";
  [key: string]: any;     // For custom fields
}

export interface UseUserTransactionsOptions {
  address: string | null;
  page?: number;
  pageSize?: number;
  filterType?: UserTransaction["type"] | "All";
  autoRefreshInterval?: number; // ms
}

interface UseUserTransactionsResult {
  transactions: UserTransaction[];
  loading: boolean;
  error: string | null;
  total: number;
  page: number;
  pageSize: number;
  refetch: () => void;
  setPage: (page: number) => void;
}

/**
 * useUserTransactions
 * Fetches a user's transaction/activity history, paginated and filterable.
 * Replace the fetch URL with your backend, subgraph, or explorer API.
 */
export function useUserTransactions(options: UseUserTransactionsOptions): UseUserTransactionsResult {
  const {
    address,
    page = 1,
    pageSize = 15,
    filterType = "All",
    autoRefreshInterval = 0,
  } = options;

  const [transactions, setTransactions] = useState<UserTransaction[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [currentPage, setCurrentPage] = useState(page);

  const fetchTxs = useCallback(async () => {
    if (!address) {
      setTransactions([]);
      setTotal(0);
      setLoading(false);
      return;
    }
    setLoading(true);
    setError(null);
    try {
      // Example endpoint—replace with your explorer, subgraph, or indexer API
      let url = `/api/transactions?address=${address}&page=${currentPage}&pageSize=${pageSize}`;
      if (filterType && filterType !== "All") url += `&type=${filterType}`;

      const res = await fetch(url);
      if (!res.ok) throw new Error("Failed to fetch transactions");
      const data = await res.json();

      setTransactions(data.transactions || []);
      setTotal(data.total || 0);
    } catch (err: any) {
      setError(err.message || "Error loading transactions");
    }
    setLoading(false);
  }, [address, currentPage, pageSize, filterType]);

  useEffect(() => {
    fetchTxs();
    let interval: NodeJS.Timeout | undefined;
    if (autoRefreshInterval > 0) {
      interval = setInterval(fetchTxs, autoRefreshInterval);
    }
    return () => {
      if (interval) clearInterval(interval);
    };
  }, [fetchTxs, autoRefreshInterval]);

  return {
    transactions,
    loading,
    error,
    total,
    page: currentPage,
    pageSize,
    refetch: fetchTxs,
    setPage: setCurrentPage,
  };
}
