import { useCallback, useEffect, useState } from "react";

// Example governance proposal type (customize fields as needed)
export interface GovernanceProposal {
  id: string;
  title: string;
  proposer: string;
  createdAt: string;
  status: "Pending" | "Approved" | "Rejected" | "Executed" | "Expired" | "Failed";
  votesFor: number;
  votesAgainst: number;
  description?: string;
  [key: string]: any;
}

// Vote input type
export interface GovernanceVoteInput {
  proposalId: string;
  voter: string;
  support: boolean; // true=for, false=against
  weight?: number;
}

// Delegation input type
export interface GovernanceDelegateInput {
  delegator: string;
  delegatee: string;
}

export interface UseGovernanceOptions {
  page?: number;
  pageSize?: number;
  status?: GovernanceProposal["status"] | "All";
  search?: string;
  autoRefreshInterval?: number;
}

interface UseGovernanceResult {
  proposals: GovernanceProposal[];
  loading: boolean;
  error: string | null;
  total: number;
  page: number;
  pageSize: number;
  refetch: () => void;
  setPage: (page: number) => void;
  vote: (input: GovernanceVoteInput) => Promise<boolean>;
  delegate: (input: GovernanceDelegateInput) => Promise<boolean>;
}

/**
 * useGovernance
 * Fetches governance proposals, supports voting, delegation, and pagination.
 * Replace fetch URLs with your API/graph/contract methods.
 */
export function useGovernance(options: UseGovernanceOptions = {}): UseGovernanceResult {
  const {
    page = 1,
    pageSize = 10,
    status = "All",
    search = "",
    autoRefreshInterval = 0,
  } = options;

  const [proposals, setProposals] = useState<GovernanceProposal[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [currentPage, setCurrentPage] = useState(page);

  const fetchProposals = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      // Example REST/GraphQL endpoint. Swap for your on-chain fetch as needed
      let url = `/api/governance/proposals?page=${currentPage}&pageSize=${pageSize}`;
      if (status && status !== "All") url += `&status=${status}`;
      if (search) url += `&search=${encodeURIComponent(search)}`;
      const res = await fetch(url);
      if (!res.ok) throw new Error("Failed to fetch governance proposals");
      const data = await res.json();
      setProposals(data.proposals || []);
      setTotal(data.total || 0);
    } catch (err: any) {
      setError(err.message || "Error loading proposals");
    }
    setLoading(false);
  }, [currentPage, pageSize, status, search]);

  // Fetch proposals on mount, change, or refetch
  useEffect(() => {
    fetchProposals();
    let interval: NodeJS.Timeout | undefined;
    if (autoRefreshInterval > 0) {
      interval = setInterval(fetchProposals, autoRefreshInterval);
    }
    return () => {
      if (interval) clearInterval(interval);
    };
  }, [fetchProposals, autoRefreshInterval]);

  // Voting logic (replace POST/fetch with contract/write logic as needed)
  const vote = useCallback(
    async (input: GovernanceVoteInput): Promise<boolean> => {
      try {
        const res = await fetch("/api/governance/vote", {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify(input),
        });
        if (!res.ok) throw new Error("Failed to submit vote");
        fetchProposals(); // Refresh proposals after voting
        return true;
      } catch (err) {
        setError((err as any).message || "Vote failed");
        return false;
      }
    },
    [fetchProposals]
  );

  // Delegation logic (replace POST/fetch with contract/write logic as needed)
  const delegate = useCallback(
    async (input: GovernanceDelegateInput): Promise<boolean> => {
      try {
        const res = await fetch("/api/governance/delegate", {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify(input),
        });
        if (!res.ok) throw new Error("Failed to delegate");
        fetchProposals(); // Refresh after delegation
        return true;
      } catch (err) {
        setError((err as any).message || "Delegation failed");
        return false;
      }
    },
    [fetchProposals]
  );

  return {
    proposals,
    loading,
    error,
    total,
    page: currentPage,
    pageSize,
    refetch: fetchProposals,
    setPage: setCurrentPage,
    vote,
    delegate,
  };
}
