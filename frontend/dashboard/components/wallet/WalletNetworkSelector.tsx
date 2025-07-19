import React from "react";
import { Globe } from "lucide-react";

interface WalletNetworkSelectorProps {
  current: string;
  networks: { name: string; chainId: number; iconUrl?: string }[];
  onSwitch: (chainId: number) => void;
}

export const WalletNetworkSelector: React.FC<WalletNetworkSelectorProps> = ({
  current,
  networks,
  onSwitch,
}) => (
  <div className="flex items-center gap-2">
    <Globe className="w-4 h-4 text-accent" />
    <select
      className="rounded border px-2 py-1 text-sm"
      value={current}
      onChange={e => onSwitch(Number(e.target.value))}
      aria-label="Select Network"
    >
      {networks.map((n) => (
        <option key={n.chainId} value={n.chainId}>
          {n.iconUrl && (
            <img src={n.iconUrl} alt={n.name} className="inline w-4 h-4 mr-1" />
          )}
          {n.name}
        </option>
      ))}
    </select>
  </div>
);
