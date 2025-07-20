import React, { createContext, useContext } from "react";

interface AnalyticsEvent {
  type: string;
  payload?: any;
}

interface AnalyticsContextValue {
  trackEvent: (event: AnalyticsEvent) => void;
}

const AnalyticsContext = createContext<AnalyticsContextValue | undefined>(undefined);

export const AnalyticsProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const trackEvent = (event: AnalyticsEvent) => {
    // Example: send to Segment, Mixpanel, or custom endpoint
    if (process.env.NODE_ENV === "development") {
      console.log("[Analytics]", event);
    }
    // TODO: Send event to analytics service
  };

  return (
    <AnalyticsContext.Provider value={{ trackEvent }}>
      {children}
    </AnalyticsContext.Provider>
  );
};

export function useAnalyticsContext() {
  const ctx = useContext(AnalyticsContext);
  if (!ctx) throw new Error("useAnalyticsContext must be used within AnalyticsProvider");
  return ctx;
}
