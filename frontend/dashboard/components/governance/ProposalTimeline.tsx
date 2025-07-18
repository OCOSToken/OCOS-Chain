import React from "react";
import { Clock } from "lucide-react";

export interface ProposalTimelineItem {
  label: string;
  date: string;
  description?: string;
  icon?: React.ReactNode;
  color?: string;
}

export interface ProposalTimelineProps {
  items: ProposalTimelineItem[];
}

export const ProposalTimeline: React.FC<ProposalTimelineProps> = ({ items }) => (
  <ol className="relative border-l-2 border-primary/20 pl-5">
    {items.map((item, idx) => (
      <li key={idx} className="mb-7 ml-3 relative">
        <span
          className={`absolute -left-6 top-0 w-5 h-5 flex items-center justify-center rounded-full border-2 bg-background border-primary/30 ${item.color ?? ""}`}
          aria-hidden="true"
        >
          {item.icon || <Clock className="w-4 h-4 text-primary" />}
        </span>
        <div>
          <div className="flex items-center gap-2">
            <span className="font-semibold">{item.label}</span>
            <span className="text-xs text-muted-foreground">{item.date}</span>
          </div>
          {item.description && (
            <div className="text-sm text-muted-foreground mt-0.5">{item.description}</div>
          )}
        </div>
      </li>
    ))}
  </ol>
);
