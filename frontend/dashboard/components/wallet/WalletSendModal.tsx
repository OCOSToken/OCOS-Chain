import React, { useState } from "react";
import { X } from "lucide-react";

interface WalletSendModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSend: (to: string, amount: number, token: string) => Promise<void>;
  tokens: { symbol: string; balance: number }[];
  defaultToken?: string;
}

export const WalletSendModal: React.FC<WalletSendModalProps> = ({
  isOpen,
  onClose,
  onSend,
  tokens,
  defaultToken,
}) => {
  const [to, setTo] = useState("");
  const [amount, setAmount] = useState<number>(0);
  const [token, setToken] = useState(defaultToken || (tokens[0]?.symbol || ""));
  const [pending, setPending] = useState(false);
  const [error, setError] = useState<string | null>(null);

  if (!isOpen) return null;

  const handleSend = async (e: React.FormEvent) => {
    e.preventDefault();
    setPending(true);
    setError(null);
    try {
      await onSend(to, amount, token);
      setTo("");
      setAmount(0);
      onClose();
    } catch (err: any) {
      setError(err.message || "Failed to send");
    }
    setPending(false);
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/40 backdrop-blur-sm" tabIndex={-1}>
      <form
        className="bg-card border shadow-xl rounded-2xl p-7 w-full max-w-sm relative flex flex-col"
        onSubmit={handleSend}
      >
        <button
          aria-label="Close"
          className="absolute top-3 right-3 rounded-full p-1 hover:bg-muted"
          onClick={onClose}
          type="button"
        >
          <X className="w-5 h-5" />
        </button>
        <h2 className="font-bold text-xl mb-3">Send Funds</h2>
        <label className="text-sm mb-1 font-semibold">Recipient Address</label>
        <input
          className="w-full border rounded px-2 py-1 mb-2"
          type="text"
          required
          value={to}
          onChange={e => setTo(e.target.value)}
        />
        <label className="text-sm mb-1 font-semibold">Token</label>
        <select
          className="w-full border rounded px-2 py-1 mb-2"
          value={token}
          onChange={e => setToken(e.target.value)}
        >
          {tokens.map(t => (
            <option key={t.symbol} value={t.symbol}>
              {t.symbol} (Balance: {t.balance})
            </option>
          ))}
        </select>
        <label className="text-sm mb-1 font-semibold">Amount</label>
        <input
          className="w-full border rounded px-2 py-1 mb-2"
          type="number"
          min={0}
          required
          value={amount || ""}
          onChange={e => setAmount(Number(e.target.value))}
        />
        {error && <span className="text-red-600 text-xs mb-2">{error}</span>}
        <button
          className="bg-primary text-white font-semibold rounded px-4 py-2 mt-2 disabled:opacity-60"
          type="submit"
          disabled={pending}
        >
          {pending ? "Sending..." : "Send"}
        </button>
      </form>
    </div>
  );
};
