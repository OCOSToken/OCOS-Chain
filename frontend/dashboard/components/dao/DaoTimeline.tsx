import React from "react";
import { CheckCircle, Clock, XCircle, Zap, ArrowUpRight, User, FileText } from "lucide-react";

export interface DaoTimelineItem {
  label: string;
  date: string;
  description?: string;
  icon?: React.ReactNode;
  color?: string; // Tailwind color class, e.g. "text-green-600"
}

export interface DaoTimelineProps {
  items: DaoTimelineItem[];
  vertical?: boolean; // For possible horizontal use in future
}

export const DaoTimeline: React.FC<DaoTimelineProps> = ({
  items,
  vertical = true,
}) => {
  return (
    <ol className={vertical ? "relative border-l-2 border-primary/20 pl-5" : "flex gap-6"}>
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
};

// Example icon mapping (to use in parent code or timeline generation)
export const TimelineIcons = {
  Submitted: <FileText className="w-4 h-4 text-blue-600" />,
  Voting: <Clock className="w-4 h-4 text-yellow-500" />,
  Approved: <CheckCircle className="w-4 h-4 text-green-600" />,
  Rejected: <XCircle className="w-4 h-4 text-red-600" />,
  Executed: <Zap className="w-4 h-4 text-primary" />,
  MemberAdded: <User className="w-4 h-4 text-green-600" />,
  TreasuryTx: <ArrowUpRight className="w-4 h-4 text-accent" />,
};
