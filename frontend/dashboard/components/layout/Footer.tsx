import React from "react";

export const Footer: React.FC = () => (
  <footer className="w-full text-center py-4 text-xs text-muted-foreground border-t bg-background">
    © {new Date().getFullYear()} OCOS Foundation. All rights reserved.  
    <span className="mx-2">|</span>
    <a href="https://ocos.io" target="_blank" rel="noopener noreferrer" className="underline hover:text-primary">
      ocos.io
    </a>
  </footer>
);
