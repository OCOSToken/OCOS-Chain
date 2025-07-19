import React, { useState } from "react";
import Link from "next/link";
import { Menu, X } from "lucide-react";

const mobileRoutes = [
  { href: "/", label: "Home" },
  { href: "/dao", label: "DAO" },
  { href: "/pools", label: "Liquidity" },
  { href: "/governance", label: "Governance" },
  { href: "/wallet", label: "Wallet" },
  { href: "/analytics", label: "Analytics" },
  { href: "/settings", label: "Settings" },
];

export const MobileDrawer: React.FC = () => {
  const [open, setOpen] = useState(false);

  return (
    <>
      <button
        className="lg:hidden fixed top-4 left-4 z-50 bg-background p-2 rounded-full shadow-md border hover:bg-accent transition"
        onClick={() => setOpen(true)}
        aria-label="Open menu"
      >
        <Menu className="w-6 h-6" />
      </button>
      {open && (
        <div className="fixed inset-0 z-50 bg-black/60 flex">
          <nav className="bg-background shadow-lg w-64 h-full flex flex-col py-8">
            <button
              className="absolute top-4 right-4 p-1 rounded hover:bg-muted"
              onClick={() => setOpen(false)}
              aria-label="Close menu"
            >
              <X className="w-6 h-6" />
            </button>
            <ul className="flex flex-col gap-4 px-6 mt-10">
              {mobileRoutes.map((route) => (
                <li key={route.href}>
                  <Link
                    href={route.href}
                    className="block py-2 px-3 rounded-lg hover:bg-accent/40 font-medium"
                    onClick={() => setOpen(false)}
                  >
                    {route.label}
                  </Link>
                </li>
              ))}
            </ul>
          </nav>
          <div className="flex-1" onClick={() => setOpen(false)} />
        </div>
      )}
    </>
  );
};
