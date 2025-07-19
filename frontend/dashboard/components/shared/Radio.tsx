import React from "react";
import { cn } from "@/lib/utils";

export interface RadioProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label: string;
  name: string;
}

export const Radio: React.FC<RadioProps> = ({ label, name, className, ...rest }) => (
  <label className="inline-flex items-center gap-2 cursor-pointer">
    <input
      type="radio"
      name={name}
      className={cn("form-radio h-4 w-4 text-primary focus:ring-primary", className)}
      {...rest}
    />
    <span className="text-sm">{label}</span>
  </label>
);
