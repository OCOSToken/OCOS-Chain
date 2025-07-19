import React from "react";
import { cn } from "@/lib/utils";

export interface TextAreaProps extends React.TextareaHTMLAttributes<HTMLTextAreaElement> {
  label?: string;
  error?: string;
}

export const TextArea: React.FC<TextAreaProps> = ({
  label,
  error,
  className,
  ...rest
}) => (
  <div className="flex flex-col gap-1">
    {label && <label className="text-sm font-medium">{label}</label>}
    <textarea
      className={cn(
        "rounded border px-3 py-2 bg-background text-foreground outline-none focus:ring-2 focus:ring-primary transition resize-y",
        error && "border-red-500",
        className
      )}
      {...rest}
    />
    {error && <span className="text-xs text-red-600">{error}</span>}
  </div>
);
