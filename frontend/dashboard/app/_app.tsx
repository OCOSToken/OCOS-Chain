import type { AppProps } from 'next/app';
import { Inter } from 'next/font/google';
import { ThemeProvider } from '@/context/ThemeContext';
import { WalletProvider } from '@/context/WalletContext';
import { Toaster } from '@/components/shared/Toaster';
import '@/styles/globals.css';

// Optional: Set up a global font
const inter = Inter({ subsets: ['latin'], variable: '--font-inter' });

export default function MyApp({ Component, pageProps }: AppProps) {
  return (
    <ThemeProvider>
      <WalletProvider>
        <main className={inter.className}>
          <Component {...pageProps} />
        </main>
        <Toaster />
      </WalletProvider>
    </ThemeProvider>
  );
}
