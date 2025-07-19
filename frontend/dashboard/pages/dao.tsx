import React from "react";
import Head from "next/head";
import { AppLayout } from "@/components/layout";
import { DaoList, DaoOverviewCard, DaoEmptyState } from "@/components/dao";

// Example mock data (replace with actual API/hook data)
const mockDaos = [
  {
    daoName: "OCOS DAO",
    avatarUrl: "/daos/ocos.png",
    members: 37,
    treasury: "$125,000",
    proposals: 18,
    activeProposals: 3,
    votesToday: 45,
    metadata: "Main OCOS on-chain DAO",
  },
  {
    daoName: "OCOS Labs",
    avatarUrl: "/daos/ocoslabs.png",
    members: 12,
    treasury: "$18,230",
    proposals: 7,
    activeProposals: 1,
    votesToday: 12,
    metadata: "Innovation & R&D DAO",
  },
  // ...add more DAOs here
];

export default function DaoExplorer() {
  // If loading from API: use SWR, React Query, or getServerSideProps
  const [selectedDao, setSelectedDao] = React.useState<typeof mockDaos[0] | null>(null);

  return (
    <AppLayout>
      <Head>
        <title>DAO Explorer | OCOS Dashboard</title>
        <meta name="description" content="Browse and join OCOS-Chain DAOs, proposals, treasury, and governance." />
      </Head>

      <section className="py-6">
        <h1 className="font-bold text-3xl mb-6">DAO Explorer</h1>
        {mockDaos.length === 0 ? (
          <DaoEmptyState
            type="dao"
            message="No DAOs available yet."
            ctaLabel="Create New DAO"
            onCta={() => alert("DAO creation coming soon!")}
          />
        ) : (
          <DaoList
            daos={mockDaos}
            onDaoSelect={(dao) => setSelectedDao(dao)}
          />
        )}
      </section>

      {selectedDao && (
        <div className="fixed inset-0 bg-black/40 z-50 flex items-center justify-center">
          <div className="bg-card rounded-xl border shadow-xl p-8 w-full max-w-xl relative">
            <button
              className="absolute top-4 right-4 text-muted-foreground hover:text-primary"
              onClick={() => setSelectedDao(null)}
              aria-label="Close"
            >
              &times;
            </button>
            <DaoOverviewCard {...selectedDao} />
          </div>
        </div>
      )}
    </AppLayout>
  );
}
