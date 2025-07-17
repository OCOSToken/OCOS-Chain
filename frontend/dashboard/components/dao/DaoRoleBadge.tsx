import React from "react";
import { ShieldCheck, User, Star, Crown, Users } from "lucide-react";

export interface DaoRoleBadgeProps {
  role: string;
}

const roleStyles: Record<string, { label: string; color: string; Icon: React.FC<any> }> = {
  Admin: {
    label: "Admin",
    color: "bg-purple-100 text-purple-700 border border-purple-300",
    Icon: ShieldCheck,
  },
  Council: {
    label: "Council",
    color: "bg-blue-100 text-blue-700 border border-blue-300",
    Icon: Users,
  },
  Member: {
    label: "Member",
    color: "bg-green-100 text-green-700 border border-green-300",
    Icon: User,
  },
  Founder: {
    label: "Founder",
    color: "bg-yellow-100 text-yellow-700 border border-yellow-300",
    Icon: Crown,
  },
  Guest: {
    label: "Guest",
    color: "bg-gray-100 text-gray-600 border border-gray-300",
    Icon: Star,
  },
  // Add more roles as needed
};

export const DaoRoleBadge: React.FC<DaoRoleBadgeProps> = ({ role }) => {
  const style = roleStyles[role] || {
    label: role,
    color: "bg-muted text-foreground border border-muted",
    Icon: User,
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
