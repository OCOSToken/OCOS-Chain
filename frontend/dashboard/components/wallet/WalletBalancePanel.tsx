import React from "react";
import { Coins } from "lucide-react";

export interface WalletTokenBalance {
  symbol: string;
  amount: number;
  usdValue?: number;
  iconUrl?: string;
}
interface WalletBalancePanelProps {
  balances: WalletTokenBalance[];
  totalUsd?: number;
}
export const WalletBalancePanel: React.FC<WalletBalancePanelProps> = ({
  balances,
  totalUsd,
}) => (
  <section className="bg-card border rounded-xl p-5 mb-5">
    <h3 className="font-bold text-lg mb-2 flex items-center gap-2">
      <Coins className="w-5 h-5 text-accent" /> Balances
    </h3>
    <div className="grid grid-cols-2 gap-3">
      {balances.map((token) => (
        <div key={token.symbol} className="flex items-center gap-2 bg-muted rounded-lg px-3 py-2">
          {token.iconUrl ? (
            <img src={token.iconUrl} alt={token.symbol} className="w-6 h-6 rounded-full border" />
          ) : (
            <Coins className="w-5 h-5 text-accent" />
          )}
          <span className="font-semibold">{token.amount}</span>
          <span className="font-mono text-xs">{token.symbol}</span>
          {token.usdValue !== undefined && (
            <span className="ml-auto text-xs text-muted-foreground">${token.usdValue.toLocaleString()}</span>
          )}
        </div>
      ))}
    </div>
    {totalUsd !== undefined && (
      <div className="mt-3 text-xs text-muted-foreground">
        Total: <span className="font-bold">${totalUsd.toLocaleString()}</span>
      </div>
    )}
  </section>
);
