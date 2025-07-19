import React from "react";

export interface WalletNft {
  id: string | number;
  imageUrl: string;
  name: string;
  collection?: string;
  link?: string;
}

interface WalletNftGalleryProps {
  nfts: WalletNft[];
}

export const WalletNftGallery: React.FC<WalletNftGalleryProps> = ({ nfts }) => (
  <section className="bg-card border rounded-xl p-5 mb-6">
    <h3 className="font-bold text-lg mb-2">NFT Gallery</h3>
    {nfts.length === 0 ? (
      <div className="text-muted-foreground text-sm text-center py-8">
        No NFTs in your wallet.
      </div>
    ) : (
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        {nfts.map((nft) => (
          <a
            key={nft.id}
            href={nft.link}
            target="_blank"
            rel="noopener noreferrer"
            className="block rounded-lg bg-muted hover:bg-accent/30 transition border"
          >
            <img
              src={nft.imageUrl}
              alt={nft.name}
              className="w-full aspect-square object-cover rounded-t-lg"
            />
            <div className="p-2">
              <div className="font-semibold text-xs truncate">{nft.name}</div>
              {nft.collection && (
                <div className="text-xs text-muted-foreground truncate">{nft.collection}</div>
              )}
            </div>
          </a>
        ))}
      </div>
    )}
  </section>
);
