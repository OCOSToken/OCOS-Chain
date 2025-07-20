import { useEffect, useState, useCallback } from "react";

// Example DAO Proposal type (customize to your protocol needs)
export interface DaoProposal {
  id: string;
  title: string;
  proposer: string;
  createdAt: string;
  status: "Pending" | "Approved" | "Rejected" | "Executed" | "Expired" | "Failed";
  votesFor: number;
  votesAgainst: number;
  description?: string;
  [key: string]: any; // For custom fields
}

export interface UseDaoProposalsOptions {
  page?: number;
  pageSize?: number;
  status?: DaoProposal["status"] | "All";
  search?: string;
  autoRefreshInterval?: number; // in ms
}

interface DaoProposalsResult {
  proposals: DaoProposal[];
  loading: boolean;
  error: string | null;
  total: number;
  page: number;
  pageSize: number;
  refetch: () => void;
  setPage: (page: number) => void;
}

/**
 * useDaoProposals
 * Fetches DAO proposals, supports filtering, pagination, and auto-refresh.
 * Replace fetch URL with your backend/graph endpoint.
 */
export function useDaoProposals(options: UseDaoProposalsOptions = {}): DaoProposalsResult {
  const {
    page = 1,
    pageSize = 10,
    status = "All",
    search = "",
    autoRefreshInterval = 0,
  } = options;

  const [proposals, setProposals] = useState<DaoProposal[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [currentPage, setCurrentPage] = useState(page);

  const fetchProposals = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      // Example REST endpoint, replace with your own API/GraphQL/on-chain fetch
      let url = `/api/dao/proposals?page=${currentPage}&pageSize=${pageSize}`;
      if (status && status !== "All") url += `&status=${status}`;
      if (search) url += `&search=${encodeURIComponent(search)}`;

      const res = await fetch(url);
      if (!res.ok) throw new Error("Failed to fetch proposals");
      const data = await res.json();

      setProposals(data.proposals || []);
      setTotal(data.total || 0);
    } catch (err: any) {
      setError(err.message || "Error loading proposals");
    }
    setLoading(false);
  }, [currentPage, pageSize, status, search]);

  // Fetch proposals on mount, page/status/search change, or refetch
  useEffect(() => {
    fetchProposals();
    // Optional: Set up auto-refresh
    let interval: NodeJS.Timeout | undefined;
    if (autoRefreshInterval > 0) {
      interval = setInterval(fetchProposals, autoRefreshInterval);
    }
    return () => {
      if (interval) clearInterval(interval);
    };
  }, [fetchProposals, autoRefreshInterval]);

  return {
    proposals,
    loading,
    error,
    total,
    page: currentPage,
    pageSize,
    refetch: fetchProposals,
    setPage: setCurrentPage,
  };
}
