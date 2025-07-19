import React, { useState } from "react";

export interface Toast {
  id: string;
  message: string;
  type?: "success" | "error" | "info";
}

export const Toaster: React.FC = () => {
  // In real apps use context/state management; here’s a simple demo:
  const [toasts, setToasts] = useState<Toast[]>([]);

  // Example: call setToasts([...toasts, { id: nanoid(), message: "Saved!", type: "success" }]);
  // Add code to remove toast after timeout.

  return (
    <div className="fixed bottom-5 right-5 z-50 flex flex-col gap-2">
      {toasts.map((toast) => (
        <div
          key={toast.id}
          className={`rounded-lg shadow px-4 py-2 font-semibold ${
            toast.type === "success"
              ? "bg-green-100 text-green-800"
              : toast.type === "error"
              ? "bg-red-100 text-red-700"
              : "bg-muted text-muted-foreground"
          }`}
        >
          {toast.message}
        </div>
      ))}
    </div>
  );
};
