import { useEffect, useState } from "react";

/**
 * useDebounce
 * Delays updating the returned value until after `delay` ms have elapsed since the last change.
 * @param value - The value to debounce (string, number, array, object)
 * @param delay - Number of milliseconds to delay (default: 300)
 * @returns Debounced value
 */
export function useDebounce<T>(value: T, delay: number = 300): T {
  const [debounced, setDebounced] = useState<T>(value);

  useEffect(() => {
    const handler = setTimeout(() => setDebounced(value), delay);
    return () => clearTimeout(handler);
  }, [value, delay]);

  return debounced;
}
