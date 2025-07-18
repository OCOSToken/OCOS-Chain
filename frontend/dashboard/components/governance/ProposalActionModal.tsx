import React, { ReactNode } from "react";
import { X } from "lucide-react";

export interface ProposalActionModalProps {
  isOpen: boolean;
  title: string;
  description?: string;
  children: ReactNode;
  onClose: () => void;
  actions?: ReactNode;
}

export const ProposalActionModal: React.FC<ProposalActionModalProps> = ({
  isOpen,
  title,
  description,
  children,
  onClose,
  actions,
}) => {
  if (!isOpen) return null;
  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/40 backdrop-blur-sm"
      aria-modal="true"
      role="dialog"
      tabIndex={-1}
      onClick={onClose}
    >
      <div
        className="bg-card border shadow-xl rounded-2xl p-6 w-full max-w-lg relative flex flex-col gap-4 animate-in fade-in-0 zoom-in-95"
        onClick={(e) => e.stopPropagation()}
        role="document"
      >
        <button
          aria-label="Close"
          className="absolute top-3 right-3 rounded-full p-1 hover:bg-muted focus:outline-none focus:ring-2 focus:ring-primary"
          onClick={onClose}
        >
          <X className="w-5 h-5" />
        </button>
        <h2 className="font-bold text-xl mb-0.5">{title}</h2>
        {description && <div className="text-muted-foreground text-sm mb-1">{description}</div>}
        <div className="flex-1">{children}</div>
        {actions && <div className="mt-4 flex gap-2 justify-end">{actions}</div>}
      </div>
    </div>
  );
};
