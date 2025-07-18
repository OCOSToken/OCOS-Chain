import React, { useState } from "react";
import { Button } from "@/components/shared/Button";
import { Swap, ArrowDown } from "lucide-react";

export interface PoolSwapFormProps {
  tokens: { symbol: string; iconUrl?: string; balance?: number }[];
  onSwap: (from: string, to: string, amount: number, slippage: number) => Promise<void> | void;
  estimateOut: (from: string, to: string, amount: number, slippage: number) => number | undefined;
  isSubmitting?: boolean;
  error?: string;
  minSwap?: number;
}

export const PoolSwapForm: React.FC<PoolSwapFormProps> = ({
  tokens,
  onSwap,
  estimateOut,
  isSubmitting = false,
  error,
  minSwap = 1,
}) => {
  const [fromIdx, setFromIdx] = useState(0);
  const [toIdx, setToIdx] = useState(1);
  const [amount, setAmount] = useState<string>("");
  const [slippage, setSlippage] = useState<number>(0.5); // %
  const [formError, setFormError] = useState<string | null>(null);

  const handleSwapDirection = () => {
    setFromIdx(toIdx);
    setToIdx(fromIdx);
    setAmount("");
  };

  const from = tokens[fromIdx], to = tokens[toIdx];
  const parsedAmount = parseFloat(amount || "");
  const estimated = !isNaN(parsedAmount) && parsedAmount > 0
    ? estimateOut(from.symbol, to.symbol, parsedAmount, slippage)
    : undefined;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!from || !to) return;
    if (isNaN(parsedAmount) || parsedAmount < minSwap) {
      setFormError(`Enter at least ${minSwap} ${from.symbol}`);
      return;
    }
    if (from.balance !== undefined && parsedAmount > from.balance) {
      setFormError(`Insufficient ${from.symbol} balance`);
      return;
    }
    try {
      await onSwap(from.symbol, to.symbol, parsedAmount, slippage);
      setFormError(null);
      setAmount("");
    } catch (err: any) {
      setFormError(err.message || "Swap failed.");
    }
  };

  return (
    <form
      onSubmit={handleSubmit}
      className="flex flex-col gap-5 bg-muted rounded-xl p-6 max-w-md mx-auto"
      autoComplete="off"
    >
      <h3 className="font-bold text-lg mb-2 flex items-center gap-2">
        <Swap className="w-5 h-5 text-primary" /> Swap Tokens
      </h3>
      <div className="flex gap-2 items-end">
        {/* From token */}
        <div className="flex-1">
          <label htmlFor="swap-from" className="font-semibold flex items-center gap-2">
            From
            {from.iconUrl && <img src={from.iconUrl} alt={from.symbol} className="w-5 h-5 rounded-full" />}
          </label>
          <select
            id="swap-from"
            value={fromIdx}
            onChange={(e) => setFromIdx(Number(e.target.value))}
            className="w-full px-2 py-1 border rounded mt-1"
            disabled={isSubmitting}
          >
            {tokens.map((t, idx) =>
              idx !== toIdx && (
                <option key={t.symbol} value={idx}>
                  {t.symbol}
                </option>
              )
            )}
          </select>
          <div className="text-xs text-muted-foreground">
            Balance: {from.balance ?? "-"}
          </div>
        </div>
        {/* Swap direction */}
        <button
          type="button"
          onClick={handleSwapDirection}
          className="rounded-full p-1 bg-background border mx-2 hover:bg-muted focus:outline-none"
          title="Swap direction"
          disabled={isSubmitting}
        >
          <ArrowDown className="w-4 h-4" />
        </button>
        {/* To token */}
        <div className="flex-1">
          <label htmlFor="swap-to" className="font-semibold flex items-center gap-2">
            To
            {to.iconUrl && <img src={to.iconUrl} alt={to.symbol} className="w-5 h-5 rounded-full" />}
          </label>
          <select
            id="swap-to"
            value={toIdx}
            onChange={(e) => setToIdx(Number(e.target.value))}
            className="w-full px-2 py-1 border rounded mt-1"
            disabled={isSubmitting}
          >
            {tokens.map((t, idx) =>
              idx !== fromIdx && (
                <option key={t.symbol} value={idx}>
                  {t.symbol}
                </option>
              )
            )}
          </select>
        </div>
      </div>
      {/* Amount input */}
      <div>
        <label htmlFor="swap-amount" className="font-semibold">
          Amount ({from.symbol})
        </label>
        <input
          id="swap-amount"
          type="number"
          min={minSwap}
          step="any"
          className="w-full px-2 py-1 border rounded mt-1 font-mono"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
          disabled={isSubmitting}
        />
      </div>
      {/* Slippage */}
      <div className="flex gap-2 items-center">
        <label htmlFor="slippage" className="text-xs font-medium text-muted-foreground">
          Slippage tolerance
        </label>
        <input
          id="slippage"
          type="number"
          min={0}
          max={5}
          step={0.1}
          className="w-16 px-2 py-1 border rounded font-mono"
          value={slippage}
          onChange={(e) => setSlippage(Number(e.target.value))}
          disabled={isSubmitting}
        />
        <span className="text-xs text-muted-foreground">%</span>
      </div>
      {/* Estimated output */}
      <div className="mb-2 text-sm text-muted-foreground">
        Estimated:{" "}
        <span className="font-semibold text-foreground">
          {isNaN(parsedAmount) || !estimated
            ? "-"
            : `${estimated} ${to.symbol}`}
        </span>
      </div>
      {formError && <div className="text-sm text-red-600">{formError}</div>}
      {error && !formError && <div className="text-sm text-red-600">{error}</div>}
      <Button
        type="submit"
        className="bg-primary text-white font-semibold mt-2"
        disabled={isSubmitting}
      >
        {isSubmitting ? "Swapping..." : "Swap"}
      </Button>
    </form>
  );
};
