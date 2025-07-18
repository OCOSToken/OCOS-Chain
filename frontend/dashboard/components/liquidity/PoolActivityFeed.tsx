import React from "react";
import {
  Swap,
  Coins,
  ArrowDownRight,
  ArrowUpRight,
  Gift,
  UserPlus,
  UserMinus,
  Zap,
} from "lucide-react";

export interface PoolActivityEvent {
  id: string | number;
  type: "swap" | "deposit" | "withdraw" | "reward" | "bridge-in" | "bridge-out" | "join" | "leave" | "custom";
  title: string;
  date: string;
  description?: string;
  amount?: number;
  symbol?: string;
  by?: string;
  link?: string;
}

interface PoolActivityFeedProps {
  events: PoolActivityEvent[];
  emptyMessage?: string;
  onEventClick?: (event: PoolActivityEvent) => void;
}

const eventIcons: Record<PoolActivityEvent["type"], React.ReactNode> = {
  swap: <Swap className="w-5 h-5 text-primary" />,
  deposit: <ArrowDownRight className="w-5 h-5 text-green-600" />,
  withdraw: <ArrowUpRight className="w-5 h-5 text-red-600" />,
  reward: <Gift className="w-5 h-5 text-yellow-500" />,
  "bridge-in": <Coins className="w-5 h-5 text-accent" />,
  "bridge-out": <Coins className="w-5 h-5 text-accent" />,
  join: <UserPlus className="w-5 h-5 text-green-700" />,
  leave: <UserMinus className="w-5 h-5 text-red-700" />,
  custom: <Zap className="w-5 h-5 text-accent" />,
};

export const PoolActivityFeed: React.FC<PoolActivityFeedProps> = ({
  events,
  emptyMessage = "No recent pool activity.",
  onEventClick,
}) => (
  <section className="bg-card border rounded-2xl shadow p-6 w-full max-w-xl mx-auto mt-8">
    <h3 className="font-bold text-lg mb-4 flex items-center gap-2">
      <Zap className="w-5 h-5 text-accent" /> Pool Activity
    </h3>
    <ul className="flex flex-col gap-3">
      {events.length === 0 ? (
        <li className="text-center text-muted-foreground">{emptyMessage}</li>
      ) : (
        events.map((event) => (
          <li
            key={event.id}
            className="flex items-start gap-3 p-2 rounded-lg hover:bg-muted/60 cursor-pointer group"
            onClick={() => event.link ? window.open(event.link, "_blank") : onEventClick?.(event)}
            tabIndex={0}
            role="button"
            aria-label={`Event: ${event.title}`}
            onKeyDown={(e) => (e.key === "Enter" || e.key === " ") && onEventClick?.(event)}
          >
            <span className="mt-0.5">{eventIcons[event.type] || <Zap className="w-5 h-5" />}</span>
            <div className="flex-1">
              <div className="flex flex-wrap items-center gap-x-2 gap-y-0.5">
                <span className="font-semibold">{event.title}</span>
                {event.amount !== undefined && event.symbol && (
                  <span className="text-xs text-primary font-mono">
                    {event.amount} {event.symbol}
                  </span>
                )}
                {event.by && (
                  <span className="text-xs text-muted-foreground">
                    by <b>{event.by}</b>
                  </span>
                )}
                <span className="text-xs text-muted-foreground ml-auto">{event.date}</span>
              </div>
              {event.description && (
                <div className="text-xs text-muted-foreground mt-0.5">{event.description}</div>
              )}
            </div>
          </li>
        ))
      )}
    </ul>
  </section>
);
