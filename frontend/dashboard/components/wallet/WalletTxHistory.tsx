import React from "react";
import { ArrowUpRight, ArrowDownRight, ExternalLink } from "lucide-react";

export interface WalletTx {
  hash: string;
  date: string; // ISO or formatted
  type: "IN" | "OUT";
  amount: number;
  symbol: string;
  toFrom: string;
  explorerUrl?: string;
}

interface WalletTxHistoryProps {
  txs: WalletTx[];
}

export const WalletTxHistory: React.FC<WalletTxHistoryProps> = ({ txs }) => (
  <section className="bg-card border rounded-xl p-5 mb-4">
    <h3 className="font-bold text-lg mb-2">Transaction History</h3>
    <ul className="flex flex-col gap-2">
      {txs.length === 0 ? (
        <li className="text-muted-foreground text-sm text-center">No transactions yet.</li>
      ) : (
        txs.map((tx) => (
          <li key={tx.hash} className="flex items-center gap-2 py-1 border-b">
            {tx.type === "IN" ? (
              <ArrowDownRight className="w-4 h-4 text-green-600" />
            ) : (
              <ArrowUpRight className="w-4 h-4 text-red-600" />
            )}
            <span className="font-mono text-xs">{tx.amount} {tx.symbol}</span>
            <span className="text-xs ml-2 text-muted-foreground">{tx.toFrom}</span>
            <span className="text-xs ml-auto">{tx.date}</span>
            {tx.explorerUrl && (
              <a
                href={tx.explorerUrl}
                target="_blank"
                rel="noopener noreferrer"
                aria-label="View on Explorer"
                className="ml-2"
              >
                <ExternalLink className="w-4 h-4 text-primary" />
              </a>
            )}
          </li>
        ))
      )}
    </ul>
  </section>
);
