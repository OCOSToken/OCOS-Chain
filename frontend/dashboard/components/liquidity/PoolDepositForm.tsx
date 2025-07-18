import React, { useState } from "react";
import { Button } from "@/components/shared/Button";
import { Coins } from "lucide-react";

export interface PoolDepositFormProps {
  tokens: { symbol: string; iconUrl?: string; balance?: number }[];
  minDeposit?: number;
  onDeposit: (amounts: Record<string, number>) => Promise<void> | void;
  isSubmitting?: boolean;
  error?: string;
}

export const PoolDepositForm: React.FC<PoolDepositFormProps> = ({
  tokens,
  minDeposit = 1,
  onDeposit,
  isSubmitting = false,
  error,
}) => {
  const [amounts, setAmounts] = useState<Record<string, string>>(
    Object.fromEntries(tokens.map((t) => [t.symbol, ""]))
  );
  const [formError, setFormError] = useState<string | null>(null);

  const handleChange = (symbol: string, value: string) => {
    setAmounts((prev) => ({ ...prev, [symbol]: value }));
    setFormError(null);
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    // Validate inputs
    const parsed: Record<string, number> = {};
    for (const t of tokens) {
      const val = parseFloat(amounts[t.symbol] || "");
      if (isNaN(val) || val < minDeposit) {
        setFormError(`Enter at least ${minDeposit} ${t.symbol}`);
        return;
      }
      if (t.balance !== undefined && val > t.balance) {
        setFormError(`Insufficient ${t.symbol} balance`);
        return;
      }
      parsed[t.symbol] = val;
    }
    try {
      await onDeposit(parsed);
      setFormError(null);
      setAmounts(Object.fromEntries(tokens.map((t) => [t.symbol, ""])));
    } catch (err: any) {
      setFormError(err.message || "Deposit failed.");
    }
  };

  return (
    <form
      onSubmit={handleSubmit}
      className="flex flex-col gap-4 bg-muted rounded-xl p-6 max-w-md mx-auto"
      autoComplete="off"
    >
      <h3 className="font-bold text-lg mb-2 flex items-center gap-2">
        <Coins className="w-5 h-5 text-primary" /> Add Liquidity
      </h3>
      {tokens.map((t) => (
        <div key={t.symbol} className="flex items-center gap-3">
          {t.iconUrl ? (
            <img
              src={t.iconUrl}
              alt={t.symbol}
              className="w-7 h-7 rounded-full border"
            />
          ) : (
            <Coins className="w-6 h-6 text-accent" />
          )}
          <label htmlFor={`amount-${t.symbol}`} className="font-semibold flex-1">
            {t.symbol}
            <span className="ml-2 text-xs text-muted-foreground">
              {t.balance !== undefined ? `Balance: ${t.balance}` : ""}
            </span>
          </label>
          <input
            id={`amount-${t.symbol}`}
            type="number"
            min={minDeposit}
            step="any"
            value={amounts[t.symbol]}
            onChange={(e) => handleChange(t.symbol, e.target.value)}
            className="w-32 px-2 py-1 border rounded bg-white/90 text-right font-mono"
            placeholder={`Amount`}
            disabled={isSubmitting}
          />
        </div>
      ))}
      {formError && (
        <div className="text-sm text-red-600">{formError}</div>
      )}
      {error && !formError && (
        <div className="text-sm text-red-600">{error}</div>
      )}
      <Button
        type="submit"
        className="bg-primary text-white font-semibold mt-2"
        disabled={isSubmitting}
      >
        {isSubmitting ? "Depositing..." : "Deposit"}
      </Button>
    </form>
  );
};
