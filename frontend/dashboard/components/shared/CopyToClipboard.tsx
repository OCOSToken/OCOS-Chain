import React, { useState } from "react";
import { Copy } from "lucide-react";

interface CopyToClipboardProps {
  text: string;
  className?: string;
}

export const CopyToClipboard: React.FC<CopyToClipboardProps> = ({ text, className }) => {
  const [copied, setCopied] = useState(false);
  const handleCopy = () => {
    navigator.clipboard.writeText(text);
    setCopied(true);
    setTimeout(() => setCopied(false), 800);
  };
  return (
    <button
      className={`flex items-center gap-1 text-xs ${className || ""}`}
      onClick={handleCopy}
      aria-label="Copy to clipboard"
      type="button"
    >
      <Copy className="w-4 h-4" />
      {copied ? "Copied!" : "Copy"}
    </button>
  );
};
