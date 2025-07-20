import React, { createContext, useContext, useState } from "react";

export interface DaoInfo {
  id: string;
  name: string;
  avatarUrl?: string;
  treasury?: string;
  memberCount?: number;
  [key: string]: any;
}

interface DaoContextValue {
  currentDao: DaoInfo | null;
  setDao: (dao: DaoInfo | null) => void;
}

const DaoContext = createContext<DaoContextValue | undefined>(undefined);

export const DaoProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [currentDao, setDao] = useState<DaoInfo | null>(null);
  return (
    <DaoContext.Provider value={{ currentDao, setDao }}>
      {children}
    </DaoContext.Provider>
  );
};

export function useDaoContext() {
  const ctx = useContext(DaoContext);
  if (!ctx) throw new Error("useDaoContext must be used within DaoProvider");
  return ctx;
}
