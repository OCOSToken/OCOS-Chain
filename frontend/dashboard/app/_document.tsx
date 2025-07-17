import Document, { Html, Head, Main, NextScript, DocumentContext } from 'next/document';

class OCOSDocument extends Document {
  static async getInitialProps(ctx: DocumentContext) {
    const initialProps = await Document.getInitialProps(ctx);
    return { ...initialProps };
  }
  render() {
    return (
      <Html lang="en" className="scroll-smooth">
        <Head>
          {/* Global site meta */}
          <meta charSet="utf-8" />
          <meta name="viewport" content="width=device-width, initial-scale=1" />
          <meta name="theme-color" content="#131313" />
          <meta name="description" content="OCOS-Chain: Professional DeFi, DAO & Web3 Dashboard" />

          {/* Favicon and app icons */}
          <link rel="icon" href="/favicon.ico" />
          <link rel="apple-touch-icon" href="/apple-touch-icon.png" />
          <link rel="manifest" href="/site.webmanifest" />

          {/* Preload fonts */}
          <link
            href="https://fonts.googleapis.com/css2?family=Inter:wght@400;600;700&display=swap"
            rel="stylesheet"
          />

          {/* Open Graph & Twitter meta */}
          <meta property="og:title" content="OCOS Dashboard" />
          <meta property="og:description" content="DAO, DeFi, NFT & Governance Interface for OCOS-Chain" />
          <meta property="og:image" content="/og-image.png" />
          <meta property="og:type" content="website" />
          <meta name="twitter:card" content="summary_large_image" />
          <meta name="twitter:title" content="OCOS Dashboard" />
          <meta name="twitter:description" content="Professional OCOS-Chain Web3 Dashboard" />
          <meta name="twitter:image" content="/og-image.png" />

          {/* Extra meta tags can be added here */}
        </Head>
        <body className="min-h-screen bg-background font-sans antialiased">
          <Main />
          <NextScript />
        </body>
      </Html>
    );
  }
}

export default OCOSDocument;
