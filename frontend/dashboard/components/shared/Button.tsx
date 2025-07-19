import React from "react";
import { cn } from "@/lib/utils";

interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "primary" | "secondary" | "outline" | "destructive" | "ghost";
  size?: "sm" | "md" | "lg";
  loading?: boolean;
}

export const Button: React.FC<ButtonProps> = ({
  variant = "primary",
  size = "md",
  loading = false,
  className,
  children,
  ...rest
}) => (
  <button
    className={cn(
      "inline-flex items-center justify-center rounded font-semibold transition px-4 py-2 focus:outline-none",
      variant === "primary" && "bg-primary text-white hover:bg-primary/90",
      variant === "secondary" && "bg-accent text-accent-foreground hover:bg-accent/80",
      variant === "outline" && "border border-primary text-primary bg-transparent hover:bg-primary/10",
      variant === "destructive" && "bg-red-600 text-white hover:bg-red-700",
      variant === "ghost" && "bg-transparent hover:bg-muted",
      size === "sm" && "text-xs px-3 py-1",
      size === "lg" && "text-lg px-6 py-3",
      className
    )}
    disabled={loading || rest.disabled}
    {...rest}
  >
    {loading && (
      <span className="mr-2 animate-spin">
        <svg className="w-4 h-4" viewBox="0 0 24 24" fill="none">
          <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
          <path className="opacity-75" d="M4 12a8 8 0 018-8v8z" fill="currentColor" />
        </svg>
      </span>
    )}
    {children}
  </button>
);
