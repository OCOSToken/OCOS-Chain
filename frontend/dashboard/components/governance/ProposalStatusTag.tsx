import React from "react";
import { CheckCircle, XCircle, Clock, ShieldCheck } from "lucide-react";

export interface ProposalStatusTagProps {
  status: string;
}

const statusConfig: Record<string, { label: string; color: string; Icon: React.FC<any> }> = {
  Pending: { label: "Pending", color: "bg-yellow-100 text-yellow-800 border border-yellow-300", Icon: Clock },
  Approved: { label: "Approved", color: "bg-green-100 text-green-800 border border-green-300", Icon: CheckCircle },
  Rejected: { label: "Rejected", color: "bg-red-100 text-red-800 border border-red-300", Icon: XCircle },
  Executed: { label: "Executed", color: "bg-blue-100 text-blue-800 border border-blue-300", Icon: ShieldCheck },
  Expired: { label: "Expired", color: "bg-gray-200 text-gray-700 border border-gray-300", Icon: Clock },
  Failed: { label: "Failed", color: "bg-gray-100 text-gray-500 border border-gray-200", Icon: XCircle },
};

export const ProposalStatusTag: React.FC<ProposalStatusTagProps> = ({ status }) => {
  const style = statusConfig[status] || {
    label: status,
    color: "bg-muted text-foreground border border-muted",
    Icon: Clock,
  };
  return (
    <span
      className={`inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs font-semibold ${style.color}`}
      title={style.label}
    >
      <style.Icon className="w-3.5 h-3.5" />
      {style.label}
    </span>
  );
};
