import React, { useState } from "react";
import { SlidersHorizontal, CheckCircle } from "lucide-react";
import { Button } from "@/components/shared/Button";

export interface ParameterChangePanelProps {
  parameters: { key: string; label: string; value: number | string; unit?: string }[];
  onSubmit: (changes: Record<string, number | string>) => Promise<void> | void;
  isSubmitting?: boolean;
}

export const ParameterChangePanel: React.FC<ParameterChangePanelProps> = ({
  parameters,
  onSubmit,
  isSubmitting = false,
}) => {
  const [changes, setChanges] = useState<Record<string, string>>(
    Object.fromEntries(parameters.map((p) => [p.key, String(p.value)]))
  );
  const [formMsg, setFormMsg] = useState<string | null>(null);

  const handleChange = (key: string, val: string) => {
    setChanges((prev) => ({ ...prev, [key]: val }));
    setFormMsg(null);
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await onSubmit(
        Object.fromEntries(
          Object.entries(changes).map(([k, v]) => [k, isNaN(Number(v)) ? v : Number(v)])
        )
      );
      setFormMsg("Parameters updated.");
    } catch (err: any) {
      setFormMsg(err.message || "Failed to update.");
    }
  };

  return (
    <form
      onSubmit={handleSubmit}
      className="bg-muted rounded-xl p-6 w-full max-w-lg mx-auto flex flex-col gap-5"
    >
      <h3 className="font-bold text-lg flex items-center gap-2 mb-1">
        <SlidersHorizontal className="w-5 h-5 text-accent" /> Update Governance Parameters
      </h3>
      {parameters.map((param) => (
        <div key={param.key} className="flex items-center gap-2">
          <label className="font-semibold flex-1">{param.label}</label>
          <input
            type="text"
            value={changes[param.key]}
            onChange={(e) => handleChange(param.key, e.target.value)}
            className="w-32 px-2 py-1 border rounded bg-white/90 text-right font-mono"
            disabled={isSubmitting}
          />
          {param.unit && <span className="text-xs text-muted-foreground">{param.unit}</span>}
        </div>
      ))}
      <Button type="submit" disabled={isSubmitting} className="bg-primary text-white font-semibold">
        {isSubmitting ? "Updating..." : "Update"}
      </Button>
      {formMsg && (
        <div className="flex items-center gap-2 mt-1 text-xs text-green-700">
          <CheckCircle className="w-4 h-4" /> {formMsg}
        </div>
      )}
    </form>
  );
};
