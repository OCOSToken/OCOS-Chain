import React from "react";
import Head from "next/head";
import { AppLayout } from "@/components/layout";
import { Spinner } from "@/components/shared/Spinner";

// Example stat cards and chart mockups (replace with real analytics API/hook)
const stats = [
  { label: "Total DAOs", value: 47, icon: "/icons/dao.svg" },
  { label: "Total TVL", value: "$4,370,000", icon: "/icons/tvl.svg" },
  { label: "Active Pools", value: 26, icon: "/icons/pools.svg" },
  { label: "On-chain Proposals", value: 123, icon: "/icons/gov.svg" },
];

const trendingPools = [
  { poolName: "OCOS/USDT", tvl: "$1,100,000", volume24h: "$230,000", apy: "17.4%" },
  { poolName: "BNB/OCOS", tvl: "$890,000", volume24h: "$120,000", apy: "12.3%" },
];

export default function AnalyticsDashboard() {
  // Example: Use SWR/React Query/API/SSR for live analytics data
  // For demonstration: static mock data

  return (
    <AppLayout>
      <Head>
        <title>Analytics | OCOS Dashboard</title>
        <meta name="description" content="Monitor TVL, protocol health, DAO and DeFi analytics for OCOS-Chain." />
      </Head>

      <section className="py-6">
        <h1 className="font-bold text-3xl mb-6">Dashboard Analytics</h1>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-6 mb-10">
          {stats.map((stat) => (
            <div key={stat.label} className="bg-card border rounded-2xl shadow p-5 flex flex-col items-center">
              {stat.icon && (
                <img src={stat.icon} alt={stat.label} className="w-8 h-8 mb-2" />
              )}
              <span className="text-2xl font-bold">{stat.value}</span>
              <span className="text-sm text-muted-foreground">{stat.label}</span>
            </div>
          ))}
        </div>
      </section>

      <section className="py-6">
        <h2 className="font-bold text-2xl mb-3">TVL & Volume Trends</h2>
        <div className="bg-card border rounded-2xl p-6 mb-10">
          {/* Replace with chart component (eg. recharts, chart.js, or custom) */}
          <div className="w-full h-60 flex items-center justify-center">
            <Spinner size={32} /> <span className="ml-3 text-muted-foreground">Chart loading...</span>
          </div>
        </div>
      </section>

      <section className="py-6">
        <h2 className="font-bold text-2xl mb-3">Trending Pools</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
          {trendingPools.map((pool) => (
            <div key={pool.poolName} className="bg-card border rounded-xl p-5 flex flex-col">
              <span className="font-bold text-lg mb-1">{pool.poolName}</span>
              <span className="text-sm mb-2 text-muted-foreground">TVL: {pool.tvl}</span>
              <span className="text-sm mb-2 text-muted-foreground">24h Volume: {pool.volume24h}</span>
              <span className="font-semibold text-primary">APY: {pool.apy}</span>
            </div>
          ))}
        </div>
      </section>
    </AppLayout>
  );
}
