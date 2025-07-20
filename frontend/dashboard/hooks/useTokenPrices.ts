import { useEffect, useState, useCallback } from "react";

export interface TokenPrice {
  symbol: string;
  price: number;         // e.g. in USD or chosen fiat
  lastUpdated: string;   // ISO string or block timestamp
  source?: string;       // API/oracle/DEX feed
}

export interface UseTokenPricesOptions {
  symbols?: string[];            // Only fetch these symbols, e.g. ["OCOS", "USDT"]
  interval?: number;             // ms, auto-refresh (default: 0 = no polling)
  baseCurrency?: string;         // "USD", "ETH", etc.
}

interface UseTokenPricesResult {
  prices: TokenPrice[];
  loading: boolean;
  error: string | null;
  refetch: () => void;
}

/**
 * useTokenPrices
 * Fetches current token prices, auto-refreshes, and is fully typed.
 * Replace fetch URL with your own oracle, API, or subgraph.
 */
export function useTokenPrices(options: UseTokenPricesOptions = {}): UseTokenPricesResult {
  const { symbols = [], interval = 0, baseCurrency = "USD" } = options;

  const [prices, setPrices] = useState<TokenPrice[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchPrices = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      // Example API endpoint, replace with your oracle/backend/graph as needed
      let url = `/api/prices?base=${baseCurrency}`;
      if (symbols.length > 0) url += `&symbols=${symbols.join(",")}`;
      const res = await fetch(url);
      if (!res.ok) throw new Error("Failed to fetch prices");
      const data = await res.json();
      setPrices(data.prices || []);
    } catch (err: any) {
      setError(err.message || "Error loading token prices");
    }
    setLoading(false);
  }, [symbols, baseCurrency]);

  useEffect(() => {
    fetchPrices();
    let poll: NodeJS.Timeout | undefined;
    if (interval > 0) {
      poll = setInterval(fetchPrices, interval);
    }
    return () => {
      if (poll) clearInterval(poll);
    };
  }, [fetchPrices, interval]);

  return {
    prices,
    loading,
    error,
    refetch: fetchPrices,
  };
}
