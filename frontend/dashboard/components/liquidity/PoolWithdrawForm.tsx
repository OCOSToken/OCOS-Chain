import React, { useState } from "react";
import { Button } from "@/components/shared/Button";
import { Coins, Minus } from "lucide-react";

export interface PoolWithdrawFormProps {
  lpTokenSymbol: string;
  lpTokenBalance: number;
  tokens: { symbol: string; iconUrl?: string; expectedOut?: number }[];
  minWithdraw?: number;
  onWithdraw: (lpAmount: number) => Promise<void> | void;
  isSubmitting?: boolean;
  error?: string;
}

export const PoolWithdrawForm: React.FC<PoolWithdrawFormProps> = ({
  lpTokenSymbol,
  lpTokenBalance,
  tokens,
  minWithdraw = 1,
  onWithdraw,
  isSubmitting = false,
  error,
}) => {
  const [lpAmount, setLpAmount] = useState<string>("");
  const [formError, setFormError] = useState<string | null>(null);

  const handleMax = () => setLpAmount(lpTokenBalance.toString());

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const val = parseFloat(lpAmount || "");
    if (isNaN(val) || val < minWithdraw) {
      setFormError(`Enter at least ${minWithdraw} ${lpTokenSymbol}`);
      return;
    }
    if (val > lpTokenBalance) {
      setFormError(`Insufficient LP balance`);
      return;
    }
    try {
      await onWithdraw(val);
      setFormError(null);
      setLpAmount("");
    } catch (err: any) {
      setFormError(err.message || "Withdraw failed.");
    }
  };

  return (
    <form
      onSubmit={handleSubmit}
      className="flex flex-col gap-4 bg-muted rounded-xl p-6 max-w-md mx-auto"
      autoComplete="off"
    >
      <h3 className="font-bold text-lg mb-2 flex items-center gap-2">
        <Minus className="w-5 h-5 text-primary" /> Withdraw Liquidity
      </h3>
      <div className="flex items-center gap-3 mb-2">
        <label htmlFor="lp-amount" className="font-semibold flex-1">
          LP Tokens
          <span className="ml-2 text-xs text-muted-foreground">
            Balance: {lpTokenBalance}
          </span>
        </label>
        <input
          id="lp-amount"
          type="number"
          min={minWithdraw}
          max={lpTokenBalance}
          step="any"
          value={lpAmount}
          onChange={(e) => setLpAmount(e.target.value)}
          className="w-32 px-2 py-1 border rounded bg-white/90 text-right font-mono"
          placeholder="Amount"
          disabled={isSubmitting}
        />
        <button
          type="button"
          className="text-xs text-primary underline ml-2"
          onClick={handleMax}
          tabIndex={0}
          aria-label="Max withdraw"
          disabled={isSubmitting}
        >
          Max
        </button>
      </div>

      <div className="mb-3">
        <span className="text-xs text-muted-foreground">You will receive:</span>
        <div className="flex flex-col gap-1 mt-1">
          {tokens.map((t) => (
            <div key={t.symbol} className="flex items-center gap-2">
              {t.iconUrl ? (
                <img src={t.iconUrl} alt={t.symbol} className="w-5 h-5 rounded-full" />
              ) : (
                <Coins className="w-5 h-5 text-accent" />
              )}
              <span className="font-semibold">{t.expectedOut ?? "-"}</span>
              <span className="font-mono text-xs">{t.symbol}</span>
            </div>
          ))}
        </div>
      </div>

      {formError && <div className="text-sm text-red-600">{formError}</div>}
      {error && !formError && <div className="text-sm text-red-600">{error}</div>}

      <Button
        type="submit"
        className="bg-primary text-white font-semibold mt-2"
        disabled={isSubmitting}
      >
        {isSubmitting ? "Withdrawing..." : "Withdraw"}
      </Button>
    </form>
  );
};
