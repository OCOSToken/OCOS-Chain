import React, { createContext, useContext, useCallback, useState } from "react";

export interface Notification {
  id: string;
  message: string;
  type?: "success" | "error" | "info" | "warning";
  duration?: number;
  title?: string;
}

interface NotificationContextValue {
  notifications: Notification[];
  show: (n: Omit<Notification, "id">) => void;
  remove: (id: string) => void;
  clear: () => void;
}

const NotificationContext = createContext<NotificationContextValue | undefined>(undefined);

const uid = () => Math.random().toString(36).slice(2, 10);

export const NotificationProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [notifications, setNotifications] = useState<Notification[]>([]);

  const show = useCallback((n: Omit<Notification, "id">) => {
    const id = uid();
    setNotifications((prev) => [...prev, { ...n, id }]);
    if (n.duration && n.duration > 0) {
      setTimeout(() => {
        setNotifications((prev) => prev.filter((x) => x.id !== id));
      }, n.duration);
    }
  }, []);

  const remove = useCallback((id: string) => {
    setNotifications((prev) => prev.filter((n) => n.id !== id));
  }, []);

  const clear = useCallback(() => setNotifications([]), []);

  return (
    <NotificationContext.Provider value={{ notifications, show, remove, clear }}>
      {children}
    </NotificationContext.Provider>
  );
};

export function useNotificationContext() {
  const ctx = useContext(NotificationContext);
  if (!ctx) throw new Error("useNotificationContext must be used within NotificationProvider");
  return ctx;
}
