import React from "react";
import Head from "next/head";
import { AppLayout } from "@/components/layout";
import { LiquidityPoolsList } from "@/components/liquidity";
import { DaoEmptyState } from "@/components/dao";

// Example mock pools (replace with API/hook/SSR data)
const mockPools = [
  {
    id: "1",
    poolName: "OCOS/USDT",
    tokens: [
      { symbol: "OCOS", iconUrl: "/tokens/ocos.svg" },
      { symbol: "USDT", iconUrl: "/tokens/usdt.svg" },
    ],
    tvl: 154200,
    apy: 19.2,
    status: "Active",
    metadata: "OCOS-Chain flagship pool",
  },
  {
    id: "2",
    poolName: "BNB/OCOS",
    tokens: [
      { symbol: "BNB", iconUrl: "/tokens/bnb.svg" },
      { symbol: "OCOS", iconUrl: "/tokens/ocos.svg" },
    ],
    tvl: 97200,
    apy: 11.7,
    status: "Active",
    metadata: "Main BNB/OCOS market",
  },
  // ...more pools
];

export default function PoolsExplorer() {
  // If loading from API: use SWR, React Query, or getServerSideProps
  const handlePoolSelect = (pool: typeof mockPools[0]) => {
    // Replace with router push to detail page or open modal
    alert(`Navigate to Pool: ${pool.poolName}`);
  };

  return (
    <AppLayout>
      <Head>
        <title>Liquidity Pools | OCOS Dashboard</title>
        <meta name="description" content="Discover OCOS-Chain liquidity pools, TVL, APY, yield, and swap opportunities." />
      </Head>

      <section className="py-6">
        <h1 className="font-bold text-3xl mb-6">Liquidity Pools</h1>
        {mockPools.length === 0 ? (
          <DaoEmptyState
            type="pool"
            message="No liquidity pools available."
            ctaLabel="Add New Pool"
            onCta={() => alert("Pool creation coming soon!")}
          />
        ) : (
          <LiquidityPoolsList
            pools={mockPools}
            onPoolSelect={handlePoolSelect}
          />
        )}
      </section>
    </AppLayout>
  );
}
