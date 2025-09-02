export const metadata = {
  title: "OCOS GPT",
  description: "Web3 + AI — OCOS ekosistem köməkçisi"
};

import "./globals.css";

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="az">
      <body className="min-h-screen bg-bg text-slate-100">{children}</body>
    </html>
  );
}
