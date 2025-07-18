import React from "react";
import {
  LineChart, Line, ResponsiveContainer, Tooltip, XAxis, YAxis, CartesianGrid, Area, AreaChart,
} from "recharts";
import { TrendingUp } from "lucide-react";

export interface PoolChartProps {
  data: { date: string; value: number }[];
  title?: string;
  yLabel?: string;
  type?: "line" | "area";
  color?: string;
}

export const PoolChart: React.FC<PoolChartProps> = ({
  data,
  title = "TVL (Total Value Locked)",
  yLabel = "TVL ($)",
  type = "area",
  color = "#3b82f6", // Tailwind blue-500
}) => (
  <section className="bg-card border rounded-xl p-4 shadow w-full mb-6 max-w-2xl mx-auto">
    <div className="flex items-center gap-2 mb-3">
      <TrendingUp className="w-5 h-5 text-accent" />
      <h3 className="font-bold text-base">{title}</h3>
    </div>
    {data.length === 0 ? (
      <div className="text-center py-8 text-muted-foreground">No chart data.</div>
    ) : (
      <ResponsiveContainer width="100%" height={240}>
        {type === "line" ? (
          <LineChart data={data}>
            <CartesianGrid strokeDasharray="3 3" stroke="#eee" />
            <XAxis dataKey="date" tick={{ fontSize: 12 }} />
            <YAxis tick={{ fontSize: 12 }} width={60} />
            <Tooltip />
            <Line type="monotone" dataKey="value" stroke={color} strokeWidth={2} dot={false} />
          </LineChart>
        ) : (
          <AreaChart data={data}>
            <defs>
              <linearGradient id="chartColor" x1="0" y1="0" x2="0" y2="1">
                <stop offset="0%" stopColor={color} stopOpacity={0.7} />
                <stop offset="100%" stopColor={color} stopOpacity={0.05} />
              </linearGradient>
            </defs>
            <CartesianGrid strokeDasharray="3 3" stroke="#eee" />
            <XAxis dataKey="date" tick={{ fontSize: 12 }} />
            <YAxis tick={{ fontSize: 12 }} width={60} />
            <Tooltip />
            <Area type="monotone" dataKey="value" stroke={color} fill="url(#chartColor)" fillOpacity={0.6} />
          </AreaChart>
        )}
      </ResponsiveContainer>
    )}
    <div className="mt-2 text-xs text-muted-foreground">{yLabel}</div>
  </section>
);
