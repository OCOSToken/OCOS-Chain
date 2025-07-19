import React, { ReactNode } from "react";
import { X } from "lucide-react";

export interface ModalProps {
  isOpen: boolean;
  title?: string;
  children: ReactNode;
  onClose: () => void;
  actions?: ReactNode;
}

export const Modal: React.FC<ModalProps> = ({
  isOpen,
  title,
  children,
  onClose,
  actions,
}) => {
  if (!isOpen) return null;
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/40">
      <div className="bg-card border shadow-xl rounded-2xl p-6 w-full max-w-md relative">
        <button
          aria-label="Close"
          className="absolute top-3 right-3 rounded-full p-1 hover:bg-muted"
          onClick={onClose}
        >
          <X className="w-5 h-5" />
        </button>
        {title && <h2 className="font-bold text-xl mb-3">{title}</h2>}
        <div>{children}</div>
        {actions && <div className="mt-4 flex gap-2 justify-end">{actions}</div>}
      </div>
    </div>
  );
};
