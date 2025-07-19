import React from "react";
import { Copy, LogOut, ExternalLink } from "lucide-react";

interface WalletAccountMenuProps {
  address: string;
  explorerUrl?: string;
  onDisconnect: () => void;
}

export const WalletAccountMenu: React.FC<WalletAccountMenuProps> = ({
  address,
  explorerUrl,
  onDisconnect,
}) => {
  const [copied, setCopied] = React.useState(false);
  const shortAddr = address.slice(0, 6) + "..." + address.slice(-4);

  const handleCopy = () => {
    navigator.clipboard.writeText(address);
    setCopied(true);
    setTimeout(() => setCopied(false), 800);
  };

  return (
    <div className="bg-card border rounded-lg shadow-lg p-3 flex flex-col gap-2 min-w-[220px]">
      <span className="font-mono text-sm mb-1">{shortAddr}</span>
      <div className="flex gap-2">
        <button
          onClick={handleCopy}
          className="rounded p-1 hover:bg-muted"
          aria-label="Copy address"
        >
          <Copy className="w-4 h-4" />
        </button>
        {explorerUrl && (
          <a
            href={explorerUrl.replace("{address}", address)}
            target="_blank"
            rel="noopener noreferrer"
            className="rounded p-1 hover:bg-muted"
            aria-label="View on Explorer"
          >
            <ExternalLink className="w-4 h-4" />
          </a>
        )}
        <button
          onClick={onDisconnect}
          className="ml-auto rounded p-1 hover:bg-red-100 text-red-600 font-bold"
        >
          <LogOut className="w-4 h-4" /> Disconnect
        </button>
      </div>
      {copied && <span className="text-green-600 text-xs mt-1">Copied!</span>}
    </div>
  );
};
