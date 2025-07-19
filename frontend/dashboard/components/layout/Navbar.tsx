import React from "react";
import Link from "next/link";
import { ThemeToggle } from "./ThemeToggle";
import { WalletConnectButton } from "../wallet/WalletConnectButton";

export const Navbar: React.FC = () => (
  <header className="w-full flex items-center justify-between px-6 py-3 border-b border-muted bg-background/80 sticky top-0 z-30 backdrop-blur">
    <Link href="/" className="flex items-center gap-2 font-bold text-xl tracking-tight">
      <img src="/logo.svg" alt="OCOS Logo" className="h-8 w-8" />
      OCOS Dashboard
    </Link>
    <div className="flex gap-3 items-center">
      <ThemeToggle />
      <WalletConnectButton
        isConnected={false}
        onConnect={() => {}}
        onDisconnect={() => {}}
      />
    </div>
  </header>
);
