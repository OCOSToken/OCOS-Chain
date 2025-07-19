import React from "react";
import { BadgeCheck } from "lucide-react";

interface WalletStatusProps {
  isConnected: boolean;
  address?: string;
  balance?: string;
  chain?: string;
  isTestnet?: boolean;
}

export const WalletStatus: React.FC<WalletStatusProps> = ({
  isConnected,
  address,
  balance,
  chain,
  isTestnet,
}) => (
  <div className="flex items-center gap-3 py-2">
    <span className={`w-3 h-3 rounded-full ${isConnected ? "bg-green-500" : "bg-gray-300"}`} />
    <span className="font-mono text-sm">
      {isConnected ? address?.slice(0, 6) + "..." + address?.slice(-4) : "Not connected"}
    </span>
    {isConnected && (
      <>
        <span className="text-xs text-muted-foreground ml-2">{balance || "-"} ETH</span>
        <span className="text-xs px-2 py-0.5 rounded bg-accent text-accent-foreground ml-2">{chain || "Unknown"}</span>
        {isTestnet && <span className="text-xs text-yellow-600 ml-2">Testnet</span>}
        <BadgeCheck className="w-4 h-4 text-primary ml-2" title="Connected" />
      </>
    )}
  </div>
);
