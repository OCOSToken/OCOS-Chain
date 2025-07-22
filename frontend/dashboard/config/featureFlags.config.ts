export interface FeatureFlags {
  enableExperimental: boolean;
  enableWalletConnect: boolean;
  enableAnalytics: boolean;
  enableAirdrop: boolean;
  enableDevTools: boolean;
  [key: string]: boolean;
}

export const featureFlags: FeatureFlags = {
  enableExperimental: false,
  enableWalletConnect: true,
  enableAnalytics: true,
  enableAirdrop: false,
  enableDevTools: process.env.NODE_ENV === "development",
};
