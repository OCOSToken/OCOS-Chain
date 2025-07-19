import React from "react";
import Head from "next/head";
import { AppLayout } from "@/components/layout";
import { DaoOverviewCard, DaoList } from "@/components/dao";
import { LiquidityPoolsList } from "@/components/liquidity";
import { WalletBalancePanel } from "@/components/wallet";
import { Spinner } from "@/components/shared/Spinner";

// Example mock data (replace with API, hooks, or SSR data)
const mockDaos = [
  {
    daoName: "OCOS DAO",
    avatarUrl: "/daos/ocos.png",
    members: 37,
    treasury: "$125,000",
    proposals: 18,
    activeProposals: 3,
    votesToday: 45,
  },
  {
    daoName: "OCOS Labs",
    members: 12,
    treasury: "$18,230",
    proposals: 7,
    activeProposals: 1,
    votesToday: 12,
  },
];

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
  },
];

const mockBalances = [
  { symbol: "USDT", amount: 3420.15, usdValue: 3420, iconUrl: "/tokens/usdt.svg" },
  { symbol: "OCOS", amount: 11000, usdValue: 1785, iconUrl: "/tokens/ocos.svg" },
];

export default function DashboardHome() {
  // Example: You can load data here using SWR, React Query, or fetch in getServerSideProps
  // For demonstration, static mock data is shown

  return (
    <AppLayout>
      <Head>
        <title>OCOS Dashboard | Home</title>
        <meta name="description" content="OCOS-Chain professional DAO, DeFi and Web3 dashboard" />
      </Head>

      <section className="py-6">
        <h1 className="font-bold text-3xl mb-3">OCOS Dashboard</h1>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-10">
          {mockDaos.map((dao) => (
            <DaoOverviewCard key={dao.daoName} {...dao} />
          ))}
        </div>
      </section>

      <section className="py-6">
        <h2 className="font-bold text-2xl mb-3">Top Liquidity Pools</h2>
        <LiquidityPoolsList pools={mockPools} onPoolSelect={(pool) => {}} />
      </section>

      <section className="py-6">
        <h2 className="font-bold text-2xl mb-3">Your Balances</h2>
        <WalletBalancePanel balances={mockBalances} totalUsd={5205} />
      </section>
    </AppLayout>
  );
}
