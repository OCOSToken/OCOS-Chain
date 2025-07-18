import React from "react";
import { Users, Star } from "lucide-react";
import { Button } from "@/components/shared/Button";

export interface Delegate {
  id: string | number;
  displayName: string;
  address: string;
  votingPower: number;
  proposalsVoted: number;
  isActive?: boolean;
}

interface DelegateListProps {
  delegates: Delegate[];
  onSelect?: (delegate: Delegate) => void;
}

export const DelegateList: React.FC<DelegateListProps> = ({ delegates, onSelect }) => (
  <section className="bg-card border rounded-2xl shadow p-6 w-full max-w-2xl mx-auto mb-6">
    <h3 className="font-bold text-lg flex items-center gap-2 mb-3">
      <Users className="w-5 h-5 text-accent" /> Delegates
    </h3>
    <table className="min-w-full text-sm">
      <thead>
        <tr className="border-b">
          <th className="px-3 py-2 text-left">Delegate</th>
          <th className="px-3 py-2 text-right">Voting Power</th>
          <th className="px-3 py-2 text-right">Proposals Voted</th>
          <th className="px-2 py-2"></th>
        </tr>
      </thead>
      <tbody>
        {delegates.length === 0 ? (
          <tr>
            <td colSpan={4} className="py-8 text-center text-muted-foreground">
              No delegates found.
            </td>
          </tr>
        ) : (
          delegates.map((d) => (
            <tr key={d.id} className="border-b hover:bg-muted/60 transition">
              <td className="px-3 py-2 font-semibold flex items-center gap-2">
                {d.displayName}
                {d.isActive && <Star className="w-4 h-4 text-yellow-500" />}
              </td>
              <td className="px-3 py-2 text-right">{d.votingPower}</td>
              <td className="px-3 py-2 text-right">{d.proposalsVoted}</td>
              <td className="px-2 py-2 text-right">
                <Button size="xs" variant="outline" onClick={() => onSelect?.(d)} className="text-xs">
                  View
                </Button>
              </td>
            </tr>
          ))
        )}
      </tbody>
    </table>
  </section>
);
