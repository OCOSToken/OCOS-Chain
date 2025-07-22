export interface AppRoute {
  path: string;
  label: string;
  icon?: string;           // Icon name for lucide or custom icons
  requiresAuth?: boolean;
  children?: AppRoute[];
}

export const appRoutes: AppRoute[] = [
  { path: "/", label: "Home", icon: "home" },
  { path: "/dao", label: "DAO", icon: "users" },
  { path: "/pools", label: "Liquidity", icon: "layers" },
  { path: "/governance", label: "Governance", icon: "file-text" },
  { path: "/wallet", label: "Wallet", icon: "wallet" },
  { path: "/analytics", label: "Analytics", icon: "bar-chart-2" },
  // Add more as needed
];
