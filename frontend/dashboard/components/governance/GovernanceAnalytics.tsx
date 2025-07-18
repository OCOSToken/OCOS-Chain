import React from "react";
import { BarChart3 } from "lucide-react";

export interface GovernanceAnalyticsProps {
  metrics: { label: string; value: number | string; unit?: string }[];
  chart?: React.ReactNode; // Optional chart (Recharts, etc)
}

export const GovernanceAnalytics: React.FC<GovernanceAnalyticsProps> = ({ metrics, chart }) => (
  <section className="bg-card border rounded-2xl shadow p-6 w-full max-w-2xl mx-auto mb-8">
    <div className="flex items-center gap-2 mb-4">
      <BarChart3 className="w-5 h-5 text-accent" />
      <h3 className="font-bold text-lg">Governance Analytics</h3>
    </div>
    <div className="grid grid-cols-2 md:grid-cols-3 gap-5 mb-6">
      {metrics.map((m, idx) => (
        <div key={idx} className="flex flex-col items-center bg-muted rounded-xl p-3">
          <span className="font-bold text-xl">{m.value}{m.unit && <span className="text-xs ml-0.5">{m.unit}</span>}</span>
          <span className="text-xs text-muted-foreground">{m.label}</span>
        </div>
      ))}
    </div>
    {chart && <div>{chart}</div>}
  </section>
);
