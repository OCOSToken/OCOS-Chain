import React, { useMemo, useState } from "react";
import { DaoOverviewCard } from "./DaoOverviewCard";
import { Input } from "@/components/shared/Input"; // Assume you have a shared Input component
import { DaoListItem } from "@/types/dao"; // Define this type in your codebase

interface DaoListProps {
  daos: DaoListItem[]; // Each DaoListItem contains the props for DaoOverviewCard + extra fields
  onDaoSelect?: (dao: DaoListItem) => void;
}

export const DaoList: React.FC<DaoListProps> = ({ daos, onDaoSelect }) => {
  const [query, setQuery] = useState("");

  // Simple search filter (name or metadata)
  const filteredDaos = useMemo(() => {
    const q = query.trim().toLowerCase();
    if (!q) return daos;
    return daos.filter(
      (dao) =>
        dao.daoName.toLowerCase().includes(q) ||
        (dao.metadata?.toLowerCase().includes(q) ?? false)
    );
  }, [daos, query]);

  return (
    <section>
      <div className="mb-6 flex items-center justify-between">
        <h2 className="text-2xl font-bold">DAOs</h2>
        <Input
          placeholder="Search DAOs..."
          className="max-w-xs"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          aria-label="Search DAOs"
        />
      </div>
      {filteredDaos.length === 0 ? (
        <div className="text-center py-12 text-muted-foreground text-lg">
          No DAOs found for your search.
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
          {filteredDaos.map((dao) => (
            <DaoOverviewCard
              key={dao.daoName}
              {...dao}
              onClick={() => onDaoSelect?.(dao)}
            />
          ))}
        </div>
      )}
    </section>
  );
};
