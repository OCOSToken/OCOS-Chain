'use client';

import React from "react";
import { Inter } from "next/font/google";
import Link from "next/link";
import { cn } from "@/lib/utils";
import { WalletProvider } from "@/context/WalletContext";
import { ThemeProvider, useTheme } from "@/context/ThemeContext";
import { Toaster } from "@/components/shared/Toaster";
import "@/styles/globals.css";

const inter = Inter({ subsets: ["latin"] });

function Navbar() {
  const { theme, toggleTheme } = useTheme();
  return (
    <header className="w-full flex items-center justify-between px-6 py-3 border-b border-muted bg-background/80 sticky top-0 z-30 backdrop-blur">
      <Link href="/" className="flex items-center gap-2 font-bold text-xl tracking-tight">
        <img src="/logo.svg" alt="OCOS Logo" className="h-8 w-8" />
        OCOS Dashboard
      </Link>
      <nav className="hidden md:flex gap-6 items-center text-muted-foreground font-medium">
        <Link href="/dao" className="hover:text-primary transition">DAO</Link>
        <Link href="/pools" className="hover:text-primary transition">Liquidity</Link>
        <Link href="/governance" className="hover:text-primary transition">Governance</Link>
        <Link href="/wallet" className="hover:text-primary transition">Wallet</Link>
        <Link href="/analytics" className="hover:text-primary transition">Analytics</Link>
      </nav>
      <div className="flex gap-3 items-center">
        <button
          onClick={toggleTheme}
          className="rounded-full p-2 hover:bg-accent transition"
          title={theme === 'dark' ? 'Switch to Light Mode' : 'Switch to Dark Mode'}
        >
          {theme === "dark" ? (
            <svg className="w-5 h-5 text-yellow-300" fill="currentColor" viewBox="0 0 20 20"><path d="M10 3a1 1 0 00-1 1v1.3a1 1 0 102 0V4a1 1 0 00-1-1zm-4.24 2.34a1 1 0 00-1.42 1.42l.91.91a1 1 0 101.42-1.42l-.91-.91zm8.48 0l-.91.91a1 1 0 101.42 1.42l.91-.91a1 1 0 10-1.42-1.42zM4 10a1 1 0 01-1-1H1.7a1 1 0 100 2H3a1 1 0 011-1zm12 0a1 1 0 011-1h1.3a1 1 0 110 2H17a1 1 0 01-1-1zm-9.66 4.24l-.91.91a1 1 0 001.42 1.42l.91-.91a1 1 0 10-1.42-1.42zm7.32.91l.91-.91a1 1 0 111.42 1.42l-.91.91a1 1 0 11-1.42-1.42zM10 16.7a1 1 0 001-1v-1.3a1 1 0 10-2 0V15.7a1 1 0 001 1z" /></svg>
          ) : (
            <svg className="w-5 h-5 text-zinc-600" fill="currentColor" viewBox="0 0 20 20"><path d="M17.293 13.293A8 8 0 016.707 2.707a1 1 0 00-1.124 1.546A6 6 0 1016 14.417a1 1 0 001.293-1.124z" /></svg>
          )}
        </button>
        {/* Wallet Connect/Status */}
        <div className="ml-2">
          {/* Replace below with your wallet connection UI */}
          <button className="bg-primary text-white px-3 py-1 rounded-lg font-semibold hover:bg-primary/90 transition">
            Connect Wallet
          </button>
        </div>
      </div>
    </header>
  );
}

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body className={cn("min-h-screen bg-background font-sans antialiased", inter.className)}>
        <ThemeProvider>
          <WalletProvider>
            <Navbar />
            <main className="flex flex-col min-h-[calc(100vh-60px)] w-full max-w-7xl mx-auto px-4 pt-6">
              {children}
            </main>
            <Toaster />
          </WalletProvider>
        </ThemeProvider>
      </body>
    </html>
  );
}
