import React, { useState, useMemo } from "react";
import { DaoRoleBadge } from "./DaoRoleBadge";
import { Button } from "@/components/shared/Button";
import { User as UserIcon, Star, ChevronDown, ChevronUp } from "lucide-react";

export interface DaoMember {
  id: string | number;
  address: string;
  displayName?: string;
  role: string; // e.g. "Member", "Council", "Admin"
  reputation?: number;
  stake?: number;
  joinedAt?: string; // ISO date
  isActive?: boolean;
}

interface DaoMembersTableProps {
  members: DaoMember[];
  showActions?: boolean;
  onInvite?: () => void;
  onRemove?: (member: DaoMember) => void;
}

export const DaoMembersTable: React.FC<DaoMembersTableProps> = ({
  members,
  showActions = false,
  onInvite,
  onRemove,
}) => {
  const [sortKey, setSortKey] = useState<"displayName" | "role" | "reputation" | "stake" | "joinedAt">("displayName");
  const [sortAsc, setSortAsc] = useState(true);

  const sortedMembers = useMemo(() => {
    return [...members].sort((a, b) => {
      const valA = a[sortKey] ?? "";
      const valB = b[sortKey] ?? "";
      if (typeof valA === "number" && typeof valB === "number") {
        return sortAsc ? valA - valB : valB - valA;
      }
      return sortAsc
        ? String(valA).localeCompare(String(valB))
        : String(valB).localeCompare(String(valA));
    });
  }, [members, sortKey, sortAsc]);

  return (
    <section className="bg-card border shadow rounded-2xl p-6 w-full overflow-x-auto">
      <div className="flex items-center justify-between mb-4">
        <h3 className="font-bold text-lg flex items-center gap-2">
          <UserIcon className="w-5 h-5 text-accent" /> Members
        </h3>
        {showActions && onInvite && (
          <Button onClick={onInvite} className="bg-primary text-white font-semibold">
            Invite Member
          </Button>
        )}
      </div>
      <table className="min-w-full text-sm">
        <thead>
          <tr className="border-b">
            <Th label="Name" sortKey="displayName" {...{ sortKey, setSortKey, sortAsc, setSortAsc }} />
            <Th label="Role" sortKey="role" {...{ sortKey, setSortKey, sortAsc, setSortAsc }} />
            <Th label="Reputation" sortKey="reputation" {...{ sortKey, setSortKey, sortAsc, setSortAsc }} />
            <Th label="Stake" sortKey="stake" {...{ sortKey, setSortKey, sortAsc, setSortAsc }} />
            <Th label="Joined" sortKey="joinedAt" {...{ sortKey, setSortKey, sortAsc, setSortAsc }} />
            {showActions && <th className="px-2 py-1 text-right">Actions</th>}
          </tr>
        </thead>
        <tbody>
          {sortedMembers.length === 0 ? (
            <tr>
              <td colSpan={showActions ? 6 : 5} className="py-6 text-center text-muted-foreground">
                No members found.
              </td>
            </tr>
          ) : (
            sortedMembers.map((member) => (
              <tr key={member.id} className="border-b hover:bg-muted/50 transition">
                <td className="px-3 py-2 font-medium flex items-center gap-2">
                  <UserIcon className="w-4 h-4 text-accent" />
                  {member.displayName || member.address.slice(0, 10) + "..."}
                  {member.isActive === false && (
                    <span className="ml-1 text-xs text-red-500">(inactive)</span>
                  )}
                </td>
                <td className="px-3 py-2">
                  <DaoRoleBadge role={member.role} />
                </td>
                <td className="px-3 py-2">
                  {member.reputation !== undefined ? (
                    <span className="inline-flex items-center gap-1">
                      <Star className="w-4 h-4 text-yellow-400" /> {member.reputation}
                    </span>
                  ) : (
                    "-"
                  )}
                </td>
                <td className="px-3 py-2">{member.stake !== undefined ? member.stake : "-"}</td>
                <td className="px-3 py-2">{member.joinedAt ? member.joinedAt.slice(0, 10) : "-"}</td>
                {showActions && (
                  <td className="px-2 py-2 text-right">
                    <Button
                      size="xs"
                      variant="destructive"
                      onClick={() => onRemove?.(member)}
                      className="text-xs"
                    >
                      Remove
                    </Button>
                  </td>
                )}
              </tr>
            ))
          )}
        </tbody>
      </table>
    </section>
  );
};

interface ThProps {
  label: string;
  sortKey: string;
  setSortKey: (k: any) => void;
  sortAsc: boolean;
  setSortAsc: (b: boolean) => void;
}
const Th: React.FC<ThProps> = ({ label, sortKey, setSortKey, sortAsc, setSortAsc }) => (
  <th
    className="px-3 py-2 cursor-pointer select-none text-left text-muted-foreground"
    onClick={() => {
      setSortAsc(sortKey === label.toLowerCase() ? !sortAsc : true);
      setSortKey(label.toLowerCase());
    }}
    scope="col"
  >
    <div className="flex items-center gap-1">
      {label}
      {sortKey === label.toLowerCase() &&
        (sortAsc ? (
          <ChevronUp className="w-3 h-3" />
        ) : (
          <ChevronDown className="w-3 h-3" />
        ))}
    </div>
  </th>
);
