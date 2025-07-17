import React from "react";
import {
  UserPlus, UserMinus, FileText, Vote, Coins, CheckCircle, XCircle, ArrowUpRight, ArrowDownRight,
  Star, Zap, MessageCircle, ShieldCheck
} from "lucide-react";

export interface DaoActivityEvent {
  id: string | number;
  type: "proposal" | "vote" | "member-join" | "member-leave" | "treasury-in" | "treasury-out" | "executed" | "rejected" | "comment" | "custom";
  title: string;
  date: string;
  description?: string;
  by?: string;
  amount?: number;
  tokenSymbol?: string;
  link?: string;
}

interface DaoActivityFeedProps {
  events: DaoActivityEvent[];
  emptyMessage?: string;
  onEventClick?: (event: DaoActivityEvent) => void;
}

const eventIcons: Record<DaoActivityEvent["type"], React.ReactNode> = {
  proposal: <FileText className="w-5 h-5 text-primary" />,
  vote: <Vote className="w-5 h-5 text-blue-600" />,
  member-join: <UserPlus className="w-5 h-5 text-green-600" />,
  member-leave: <UserMinus className="w-5 h-5 text-red-600" />,
  treasury-in: <ArrowDownRight className="w-5 h-5 text-green-600" />,
  treasury-out: <ArrowUpRight className="w-5 h-5 text-red-600" />,
  executed: <CheckCircle className="w-5 h-5 text-green-700" />,
  rejected: <XCircle className="w-5 h-5 text-red-600" />,
  comment: <MessageCircle className="w-5 h-5 text-accent" />,
  custom: <Star className="w-5 h-5 text-yellow-400" />,
};

export const DaoActivityFeed: React.FC<DaoActivityFeedProps> = ({
  events,
  emptyMessage = "No recent activity.",
  onEventClick,
}) => (
  <section className="bg-card border rounded-2xl shadow p-6 w-full max-w-xl mx-auto">
    <h3 className="font-bold text-lg mb-4 flex items-center gap-2">
      <Zap className="w-5 h-5 text-accent" /> Activity Feed
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
            <span className="mt-0.5">{eventIcons[event.type] || <Star className="w-5 h-5" />}</span>
            <div className="flex-1">
              <div className="flex flex-wrap items-center gap-x-2 gap-y-0.5">
                <span className="font-semibold">{event.title}</span>
                {event.amount !== undefined && event.tokenSymbol && (
                  <span className="text-xs text-green-700 font-mono">
                    {event.amount} {event.tokenSymbol}
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
