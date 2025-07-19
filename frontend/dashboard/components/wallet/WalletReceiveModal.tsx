import React from "react";
import { X, Copy } from "lucide-react";
import QRCode from "react-qr-code"; // Make sure to install: npm i react-qr-code

interface WalletReceiveModalProps {
  isOpen: boolean;
  address: string;
  onClose: () => void;
  networkName?: string;
}

export const WalletReceiveModal: React.FC<WalletReceiveModalProps> = ({
  isOpen,
  address,
  onClose,
  networkName,
}) => {
  const [copied, setCopied] = React.useState(false);

  const handleCopy = () => {
    navigator.clipboard.writeText(address);
    setCopied(true);
    setTimeout(() => setCopied(false), 1000);
  };

  if (!isOpen) return null;
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/40 backdrop-blur-sm" tabIndex={-1}>
      <div
        className="bg-card border shadow-xl rounded-2xl p-7 w-full max-w-sm relative flex flex-col items-center"
        onClick={(e) => e.stopPropagation()}
        role="dialog"
        aria-modal="true"
      >
        <button
          aria-label="Close"
          className="absolute top-3 right-3 rounded-full p-1 hover:bg-muted"
          onClick={onClose}
        >
          <X className="w-5 h-5" />
        </button>
        <h2 className="font-bold text-xl mb-3">Receive Funds</h2>
        <QRCode value={address} size={136} className="mb-4 border rounded bg-white p-2" />
        <div className="text-xs text-muted-foreground mb-2">{networkName || "Network"} Address:</div>
        <div className="font-mono text-center text-sm break-all mb-2">{address}</div>
        <button
          onClick={handleCopy}
          className="mt-1 bg-primary text-white font-semibold rounded px-4 py-1.5 flex items-center gap-2"
        >
          <Copy className="w-4 h-4" /> {copied ? "Copied!" : "Copy Address"}
        </button>
      </div>
    </div>
  );
};
