import React from "react";
import { cn } from "@/lib/utils";

export interface CheckboxProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label?: string;
}

export const Checkbox: React.FC<CheckboxProps> = ({ label, className, ...rest }) => (
  <label className="inline-flex items-center gap-2 cursor-pointer">
    <input
      type="checkbox"
      className={cn("form-checkbox h-4 w-4 text-primary focus:ring-primary", className)}
      {...rest}
    />
    {label && <span className="text-sm">{label}</span>}
  </label>
);
