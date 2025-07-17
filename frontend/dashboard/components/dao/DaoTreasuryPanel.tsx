import React from "react";
import { Coins, ArrowDownRight, ArrowUpRight, Info } from "lucide-react";
import { Button } from "@/components/shared/Button";

export interface TreasuryToken {
  symbol: string;
  balance: number;
  iconUrl?: string;
  usdValue?: number;
}

export interface TreasuryTx {
  id: string | number;
  date: string; // ISO or formatted string
  type: "IN" | "OUT";
  amount: number;
  symbol: string;
  counterparty: string;
  description?: string;
}

export interface DaoTreasuryPanelProps {
  tokens: TreasuryToken[];
  totalUsd?: number;
  recentTxs?: TreasuryTx[];
  onSendFunds?: () => void;
  onReceiveFunds?: () => void;
  onTxClick?: (tx: TreasuryTx) => void;
}

export const DaoTreasuryPanel: React.FC<DaoTreasuryPanelProps> = ({
  tokens,
  totalUsd,
  recentTxs = [],
  onSendFunds,
  onReceiveFunds,
  onTxClick,
}) => {
  return (
    <section className="bg-card border shadow rounded-2xl p-8 w-full max-w-2xl mx-auto mb-8">
      <div className="flex items-center gap-3 mb-3">
        <Coins className="w-6 h-6 text-accent" />
        <h2 className="font-extrabold text-xl flex-1">Treasury</h2>
        {onSendFunds && (
          <Button className="bg-primary text-white" onClick={onSendFunds} size="sm">
            Send
          </Button>
        )}
        {onReceiveFunds && (
          <Button variant="outline" onClick={onReceiveFunds} size="sm">
            Receive
          </Button>
        )}
      </div>
      <div className="mb-5">
        <div className="text-3xl font-extrabold text-primary">
          {totalUsd !== undefined ? `$${totalUsd.toLocaleString()}` : "—"}
        </div>
        <span className="text-xs text-muted-foreground">Total Value (USD)</span>
      </div>
      <div className="grid grid-cols-2 gap-4 mb-6">
        {tokens.length === 0 ? (
          <span className="col-span-2 text-center text-muted-foreground">No tokens in treasury.</span>
        ) : (
          tokens.map((t) => (
            <div
              key={t.symbol}
              className="flex items-center gap-2 bg-muted rounded-lg px-3 py-2"
              title={t.symbol}
            >
              {t.iconUrl ? (
                <img
                  src={t.iconUrl}
                  alt={t.symbol}
                  className="w-6 h-6 rounded-full border"
                />
              ) : (
                <Coins className="w-5 h-5 text-accent" />
              )}
              <span className="font-semibold text-base">{t.balance}</span>
              <span className="font-mono text-xs">{t.symbol}</span>
              {t.usdValue !== undefined && (
                <span className="ml-auto text-xs text-muted-foreground">
                  ${t.usdValue.toLocaleString()}
                </span>
              )}
            </div>
          ))
        )}
      </div>
      <div>
        <div className="flex items-center gap-2 mb-2">
          <Info className="w-4 h-4 text-primary" />
          <span className="font-bold text-sm">Recent Treasury Activity</span>
        </div>
        {recentTxs.length === 0 ? (
          <span className="text-muted-foreground text-xs">No recent treasury activity.</span>
        ) : (
          <ul className="flex flex-col gap-2">
            {recentTxs.slice(0, 6).map((tx) => (
              <li
                key={tx.id}
                className="flex items-center gap-2 px-2 py-1 rounded cursor-pointer hover:bg-muted/60 transition"
                onClick={() => onTxClick?.(tx)}
                tabIndex={0}
                aria-label={`Tx: ${tx.type === "IN" ? "Received" : "Sent"} ${tx.amount} ${tx.symbol}`}
              >
                {tx.type === "IN" ? (
                  <ArrowDownRight className="w-4 h-4 text-green-600" />
                ) : (
                  <ArrowUpRight className="w-4 h-4 text-red-600" />
                )}
                <span className="font-semibold">{tx.amount}</span>
                <span className="font-mono text-xs">{tx.symbol}</span>
                <span className="text-xs ml-2 text-muted-foreground">{tx.counterparty}</span>
                <span className="text-xs ml-auto">{tx.date}</span>
              </li>
            ))}
          </ul>
        )}
      </div>
    </section>
  );
};
