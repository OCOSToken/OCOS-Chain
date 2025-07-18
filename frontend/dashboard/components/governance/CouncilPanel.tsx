import React from "react";
import { Users, CheckCircle } from "lucide-react";

export interface CouncilMember {
  id: string | number;
  displayName: string;
  role: string;
  since: string;
  active?: boolean;
}

export interface CouncilPanelProps {
  members: CouncilMember[];
}

export const CouncilPanel: React.FC<CouncilPanelProps> = ({ members }) => (
  <section className="bg-card border rounded-2xl shadow p-6 w-full max-w-2xl mx-auto mb-6">
    <h3 className="font-bold text-lg flex items-center gap-2 mb-3">
      <Users className="w-5 h-5 text-accent" /> Council Members
    </h3>
    <table className="min-w-full text-sm">
      <thead>
        <tr className="border-b">
          <th className="px-3 py-2 text-left">Member</th>
          <th className="px-3 py-2 text-left">Role</th>
          <th className="px-3 py-2 text-right">Since</th>
          <th className="px-2 py-2"></th>
        </tr>
      </thead>
      <tbody>
        {members.length === 0 ? (
          <tr>
            <td colSpan={4} className="py-8 text-center text-muted-foreground">
              No council members found.
            </td>
          </tr>
        ) : (
          members.map((m) => (
            <tr key={m.id} className="border-b hover:bg-muted/60 transition">
              <td className="px-3 py-2 font-semibold">{m.displayName}</td>
              <td className="px-3 py-2">{m.role}</td>
              <td className="px-3 py-2 text-right">{m.since}</td>
              <td className="px-2 py-2 text-right">
                {m.active && <CheckCircle className="w-4 h-4 text-green-600" />}
              </td>
            </tr>
          ))
        )}
      </tbody>
    </table>
  </section>
);
