import React from "react";

export const Spinner: React.FC<{ size?: number }> = ({ size = 24 }) => (
  <svg
    className="animate-spin text-primary"
    width={size}
    height={size}
    fill="none"
    viewBox="0 0 24 24"
    aria-label="Loading"
  >
    <circle
      className="opacity-25"
      cx="12"
      cy="12"
      r="10"
      stroke="currentColor"
      strokeWidth="4"
    />
    <path
      className="opacity-75"
      d="M4 12a8 8 0 018-8v8z"
      fill="currentColor"
    />
  </svg>
);
