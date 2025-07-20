import { useCallback, useState } from "react";

// Notification type
export interface Notification {
  id: string;
  message: string;
  type?: "success" | "error" | "info" | "warning";
  duration?: number; // ms, auto-hide after duration
  title?: string;
}

// Hook return type
interface UseNotificationsResult {
  notifications: Notification[];
  show: (n: Omit<Notification, "id">) => void;
  remove: (id: string) => void;
  clear: () => void;
}

// Simple unique id generator (replace with nanoid/uuid if you prefer)
const uid = () => Math.random().toString(36).slice(2, 10);

/**
 * useNotifications
 * Global notification (toast) management hook.
 * Plug into NotificationCenter or Toaster UI component.
 */
export function useNotifications(): UseNotificationsResult {
  const [notifications, setNotifications] = useState<Notification[]>([]);

  // Show notification (auto-removes after duration if provided)
  const show = useCallback((n: Omit<Notification, "id">) => {
    const id = uid();
    setNotifications((prev) => [...prev, { ...n, id }]);

    if (n.duration && n.duration > 0) {
      setTimeout(() => {
        setNotifications((prev) => prev.filter((x) => x.id !== id));
      }, n.duration);
    }
  }, []);

  // Remove by id
  const remove = useCallback((id: string) => {
    setNotifications((prev) => prev.filter((n) => n.id !== id));
  }, []);

  // Clear all
  const clear = useCallback(() => {
    setNotifications([]);
  }, []);

  return {
    notifications,
    show,
    remove,
    clear,
  };
}
