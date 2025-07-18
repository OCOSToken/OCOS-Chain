import React from "react";
import { BarChartHorizontal, CheckCircle, XCircle, User, Vote } from "lucide-react";

export interface GovernanceOverviewPanelProps {
  proposalCount: number;
  activeCount: number;
  turnout: number;
  approved: number;
  rejected: number;
  totalVoters?: number;
}

export const GovernanceOverviewPanel: React.FC<GovernanceOverviewPanelProps> = ({
  proposalCount,
  activeCount,
  turnout,
  approved,
  rejected,
  totalVoters,
}) => (
  <section className="bg-card border rounded-2xl shadow p-6 mb-6 w-full max-w-2xl mx-auto">
    <div className="flex items-center gap-2 mb-3">
      <BarChartHorizontal className="w-5 h-5 text-accent" />
      <h3 className="font-bold text-lg flex-1">Governance Overview</h3>
    </div>
    <div className="grid grid-cols-2 sm:grid-cols-3 gap-5">
      <Stat
        icon={<Vote className="w-5 h-5 text-accent" />}
        label="Total Proposals"
        value={proposalCount}
      />
      <Stat
        icon={<User className="w-5 h-5 text-accent" />}
        label="Active Proposals"
        value={activeCount}
      />
      <Stat
        icon={<CheckCircle className="w-5 h-5 text-green-600" />}
        label="Approved"
        value={approved}
      />
      <Stat
        icon={<XCircle className="w-5 h-5 text-red-600" />}
        label="Rejected"
        value={rejected}
      />
      <Stat
        icon={<BarChartHorizontal className="w-5 h-5 text-primary" />}
        label="Turnout"
        value={`${turnout}%`}
      />
      {totalVoters !== undefined && (
        <Stat
          icon={<User className="w-5 h-5 text-accent" />}
          label="Voters"
          value={totalVoters}
        />
      )}
    </div>
  </section>
);

interface StatProps {
  icon: React.ReactNode;
  label: string;
  value: React.ReactNode;
}
const Stat: React.FC<StatProps> = ({ icon, label, value }) => (
  <div className="flex flex-col items-center p-2 gap-1">
    <div>{icon}</div>
    <div className="font-semibold text-lg">{value}</div>
    <div className="text-xs text-muted-foreground">{label}</div>
  </div>
);
