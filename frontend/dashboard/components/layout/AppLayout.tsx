import React, { ReactNode } from "react";
import { Sidebar } from "./Sidebar";
import { Navbar } from "./Navbar";
import { Footer } from "./Footer";
import { NotificationCenter } from "./NotificationCenter";
import { MobileDrawer } from "./MobileDrawer";

interface AppLayoutProps {
  children: ReactNode;
}

export const AppLayout: React.FC<AppLayoutProps> = ({ children }) => (
  <div className="min-h-screen flex flex-col bg-background font-sans text-foreground">
    {/* Top Navbar */}
    <Navbar />

    <div className="flex flex-1">
      {/* Sidebar for desktop, mobile drawer for small screens */}
      <aside className="hidden lg:block w-64">
        <Sidebar />
      </aside>
      <MobileDrawer /> {/* Handles mobile sidebar/navigation */}

      {/* Main content area */}
      <main className="flex-1 px-4 md:px-8 py-8 max-w-7xl w-full mx-auto overflow-x-auto">
        {children}
      </main>
    </div>

    {/* Footer */}
    <Footer />

    {/* Notification Center */}
    <NotificationCenter />
  </div>
);
