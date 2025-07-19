import React from "react";
import Head from "next/head";
import { AppLayout } from "@/components/layout";
import { GovernanceProposalList, GovernanceEmptyState } from "@/components/governance";

// Example mock proposals (replace with SSR/hook/API data)
const mockProposals = [
  {
    id: "42",
    title: "Adopt OCOS Treasury Upgrade",
    proposer: "Ocoshy.eth",
    createdAt: "2024-07-20",
    status: "Pending",
    votesFor: 118,
    votesAgainst: 12,
  },
  {
    id: "41",
    title: "Increase Pool Rewards for LPs",
    proposer: "DAObot",
    createdAt: "2024-07-15",
    status: "Approved",
    votesFor: 164,
    votesAgainst: 9,
  },
  // ...add more
];

export default function GovernanceExplorer() {
  // If loading from API: use SWR, React Query, or getServerSideProps
  const [selectedProposal, setSelectedProposal] = React.useState<typeof mockProposals[0] | null>(null);

  return (
    <AppLayout>
      <Head>
        <title>Governance | OCOS Dashboard</title>
        <meta name="description" content="Explore, vote, and follow governance proposals for OCOS-Chain." />
      </Head>

      <section className="py-6">
        <h1 className="font-bold text-3xl mb-6">Governance Proposals</h1>
        {mockProposals.length === 0 ? (
          <GovernanceEmptyState
            message="No governance proposals found."
            ctaLabel="Create Proposal"
            onCta={() => alert("Proposal creation coming soon!")}
          />
        ) : (
          <GovernanceProposalList
            proposals={mockProposals}
            onProposalSelect={setSelectedProposal}
          />
        )}
      </section>

      {/* Optional: Modal or drawer for selected proposal details */}
      {selectedProposal && (
        <div className="fixed inset-0 bg-black/40 z-50 flex items-center justify-center">
          <div className="bg-card rounded-xl border shadow-xl p-8 w-full max-w-xl relative">
            <button
              className="absolute top-4 right-4 text-muted-foreground hover:text-primary"
              onClick={() => setSelectedProposal(null)}
              aria-label="Close"
            >
              &times;
            </button>
            {/* Replace below with <GovernanceProposalDetails {...selectedProposal} /> */}
            <div>
              <h2 className="font-bold text-xl mb-2">{selectedProposal.title}</h2>
              <p className="mb-1 text-sm">By: {selectedProposal.proposer}</p>
              <p className="mb-2 text-xs text-muted-foreground">Created: {selectedProposal.createdAt}</p>
              <div className="flex gap-4 mt-2">
                <span className="text-green-700 font-semibold text-sm">
                  +{selectedProposal.votesFor} For
                </span>
                <span className="text-red-600 font-semibold text-sm">
                  -{selectedProposal.votesAgainst} Against
                </span>
              </div>
              <p className="mt-4 text-sm text-primary">Status: {selectedProposal.status}</p>
            </div>
          </div>
        </div>
      )}
    </AppLayout>
  );
}
