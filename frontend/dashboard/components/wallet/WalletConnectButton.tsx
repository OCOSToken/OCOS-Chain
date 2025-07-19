import React from "react";
import { LogIn, LogOut, Copy } from "lucide-react";

interface WalletConnectButtonProps {
  isConnected: boolean;
  address?: string;
  onConnect: () => void;
  onDisconnect: () => void;
  isLoading?: boolean;
  explorerUrl?: string;
}

export const WalletConnectButton: React.FC<WalletConnectButtonProps> = ({
  isConnected,
  address,
  onConnect,
  onDisconnect,
  isLoading = false,
  explorerUrl,
}) => {
  const [copied, setCopied] = React.useState(false);

  const handleCopy = (e: React.MouseEvent) => {
    e.stopPropagation();
    if (address) {
      navigator.clipboard.writeText(address);
      setCopied(true);
      setTimeout(() => setCopied(false), 1000);
    }
  };

  const shortAddr = (addr?: string) =>
    addr ? addr.slice(0, 6) + "..." + addr.slice(-4) : "";

  return (
    <button
      className={`flex items-center gap-2 px-4 py-2 rounded-lg font-semibold shadow-sm transition
        ${isConnected ? "bg-muted text-foreground border border-primary" : "bg-primary text-white"}
        hover:shadow-md focus:outline-none focus:ring-2 focus:ring-primary`}
      onClick={isConnected ? onDisconnect : onConnect}
      disabled={isLoading}
      aria-label={isConnected ? "Disconnect Wallet" : "Connect Wallet"}
      type="button"
    >
      {isConnected ? <LogOut className="w-4 h-4" /> : <LogIn className="w-4 h-4" />}
      {isLoading ? (
        <span className="animate-pulse">Connecting...</span>
      ) : isConnected && address ? (
        <span className="flex items-center gap-1">
          {shortAddr(address)}
          <span
            title="Copy address"
            className="hover:bg-accent rounded-full p-1 ml-1 transition"
            onClick={handleCopy}
            tabIndex={0}
            role="button"
            onKeyDown={(e) => (e.key === "Enter" || e.key === " ") && handleCopy(e as any)}
          >
            <Copy className="w-4 h-4 text-muted-foreground" />
          </span>
          {copied && <span className="text-xs text-green-600 ml-1">Copied!</span>}
          {explorerUrl && (
            <a
              href={explorerUrl.replace("{address}", address)}
              target="_blank"
              rel="noopener noreferrer"
              className="ml-2 text-xs underline text-primary hover:text-primary/80"
              onClick={(e) => e.stopPropagation()}
            >
              Explorer
            </a>
          )}
        </span>
      ) : (
        <span>Connect Wallet</span>
      )}
    </button>
  );
};
