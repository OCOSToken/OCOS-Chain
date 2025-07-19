import React from "react";
import Link from "next/link";
import { Home, Layers, Users, BarChart2, FileText, Wallet, Settings } from "lucide-react";

const routes = [
  { href: "/", label: "Home", icon: <Home className="w-5 h-5" /> },
  { href: "/dao", label: "DAO", icon: <Users className="w-5 h-5" /> },
  { href: "/pools", label: "Liquidity", icon: <Layers className="w-5 h-5" /> },
  { href: "/governance", label: "Governance", icon: <FileText className="w-5 h-5" /> },
  { href: "/wallet", label: "Wallet", icon: <Wallet className="w-5 h-5" /> },
  { href: "/analytics", label: "Analytics", icon: <BarChart2 className="w-5 h-5" /> },
  { href: "/settings", label: "Settings", icon: <Settings className="w-5 h-5" /> },
];

export const Sidebar: React.FC = () => (
  <nav className="h-full flex flex-col py-8 bg-background border-r gap-2">
    <div className="flex items-center gap-2 px-6 mb-8 font-extrabold text-xl tracking-tight">
      <img src="/logo.svg" alt="OCOS Logo" className="h-8 w-8" />
      OCOS Dashboard
    </div>
    <ul className="flex-1 flex flex-col gap-2">
      {routes.map(({ href, label, icon }) => (
        <li key={href}>
          <Link
            href={href}
            className="flex items-center gap-3 px-6 py-2 rounded-lg hover:bg-accent/40 font-medium transition text-muted-foreground"
          >
            {icon}
            <span>{label}</span>
          </Link>
        </li>
      ))}
    </ul>
    <div className="px-6 mt-auto mb-3">
      <span className="text-xs text-muted-foreground">v1.0 • OCOS Foundation</span>
    </div>
  </nav>
);
