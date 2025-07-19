import React from "react";
import Head from "next/head";
import { AppLayout } from "@/components/layout";
import {
  WalletConnectButton,
  WalletStatus,
  WalletBalancePanel,
  WalletTxHistory,
  WalletNftGallery,
  WalletNetworkSelector,
} from "@/components/wallet";
import { Spinner } from "@/components/shared/Spinner";

// Example mock data (replace with real API/hook data)
const mockAddress = "0x5A97...5E98";
const mockNetwork = "OCOS Mainnet";
const mockNetworks = [
  { name: "OCOS Mainnet", chainId: 1 },
  { name: "BNB Chain", chainId: 56 },
  { name: "Polygon", chainId: 137 },
];
const mockBalances = [
  { symbol: "USDT", amount: 3420.15, usdValue: 3420, iconUrl: "/tokens/usdt.svg" },
  { symbol: "OCOS", amount: 11000, usdValue: 1785, iconUrl: "/tokens/ocos.svg" },
];
const mockTxs = [
  {
    hash: "0xabc123",
    date: "2024-07-21",
    type: "IN",
    amount: 420,
    symbol: "USDT",
    toFrom: "Binance",
    explorerUrl: "https://explorer.ocos.io/tx/0xabc123",
  },
  {
    hash: "0xdef789",
    date: "2024-07-20",
    type: "OUT",
    amount: 250,
    symbol: "OCOS",
    toFrom: "DAO Treasury",
    explorerUrl: "https://explorer.ocos.io/tx/0xdef789",
  },
];
const mockNfts = [
  {
    id: "nft1",
    imageUrl: "/nfts/ocos-art1.png",
    name: "Ocos Genesis #1",
    collection: "Ocos Originals",
    link: "https://opensea.io/assets/ocos/1",
  },
  {
    id: "nft2",
    imageUrl: "/nfts/ocos-art2.png",
    name: "Ocos Genesis #2",
    collection: "Ocos Originals",
    link: "https://opensea.io/assets/ocos/2",
  },
];

export default function WalletDashboard() {
  // You can use useWallet, useAccount, or getServerSideProps for real data
  const [connected, setConnected] = React.useState(true); // Set false if user not connected
  const [network, setNetwork] = React.useState("OCOS Mainnet");
  const [address, setAddress] = React.useState(mockAddress);

  // Example handler (replace with wallet connect logic)
  const handleConnect = () => setConnected(true);
  const handleDisconnect = () => setConnected(false);
  const handleNetworkSwitch = (chainId: number) => {
    const selected = mockNetworks.find((n) => n.chainId === chainId);
    if (selected) setNetwork(selected.name);
  };

  return (
    <AppLayout>
      <Head>
        <title>Wallet | OCOS Dashboard</title>
        <meta name="description" content="Manage your assets, view transactions, and explore NFTs on OCOS-Chain." />
      </Head>

      <section className="py-6">
        <h1 className="font-bold text-3xl mb-3">Your Wallet</h1>
        <div className="flex flex-col md:flex-row md:items-center gap-4 mb-6">
          <WalletConnectButton
            isConnected={connected}
            address={address}
            onConnect={handleConnect}
            onDisconnect={handleDisconnect}
            explorerUrl="https://explorer.ocos.io/address/{address}"
          />
          <WalletStatus
            isConnected={connected}
            address={address}
            balance={"14,420"}
            chain={network}
            isTestnet={network !== "OCOS Mainnet"}
          />
          <WalletNetworkSelector
            current={mockNetworks.find((n) => n.name === network)?.chainId ?? 1}
            networks={mockNetworks}
            onSwitch={handleNetworkSwitch}
          />
        </div>

        <WalletBalancePanel balances={mockBalances} totalUsd={5205} />

        <div className="grid grid-cols-1 md:grid-cols-2 gap-8 mt-8">
          <WalletTxHistory txs={mockTxs} />
          <WalletNftGallery nfts={mockNfts} />
        </div>
      </section>
    </AppLayout>
  );
}
