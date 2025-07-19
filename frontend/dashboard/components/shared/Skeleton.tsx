import React from "react";
export const Skeleton: React.FC<{ width?: string; height?: string; className?: string }> = ({
  width = "100%",
  height = "1.25rem",
  className = "",
}) => (
  <div
    className={`animate-pulse bg-muted rounded ${className}`}
    style={{ width, height }}
  />
);
