# OCOS-Chain: NFT Modulu

**ERC-721 / ERC-1155 Uyğun | Metadata | Royalty | Kolleksiya | Marketplace | İdarəetmə | Hadisələr**

---

## Ümumi Baxış

`/nft` modulu OCOS-Chain üçün tam funksional, modulyar və auditə uyğun NFT infrastrukturu təqdim edir.  
Bu modul ERC-721 və ERC-1155 uyğun NFT-lər, metadata, kolleksiya və royalty idarəetməsi, DAO əsaslı marketplace funksiyaları və həm yaradıcılar, həm də kolleksiyaçılar üçün idarəetmə imkanı təqdim edir.

---

## Xüsusiyyətlər

- **NFT Ledger:** Sahibliyin idarəsi, transfer, yandırma (burn), mint və approval sistemi  
- **Metadata Reyestri:** On-chain və IPFS uyğun metadata strukturları, genişlənə bilən atributlarla  
- **Royalty İdarəetməsi:** Token və kolleksiya səviyyəsində royalty (ERC-2981 tərzində)  
- **Kolleksiyalar:** NFT-lərin atribut, yaradıcı və təsdiqləmə əsaslı qruplaşdırılması  
- **Marketplace Məntiqi:** NFT-lərin qiymətlə list edilməsi, alınması və satılması  
- **Auction Mexanizmi:** İngilis, Hollandiya və sealed-bid auksionları ilə DAO inteqrasiyası  
- **İdarəetmə:** Weighted vote, təmsilçilik (delegation), təkliflərin icrası və NFT əsaslı DAO-lar  
- **Hadisələr:** İzləmə, analitika və indeksasiya üçün zəncir üstü event log-lar  
- **Tiplər və Xətalar:** SDK və kontrakt inteqrasiyası üçün təmiz və modulyar strukturlar

---

## Qovluq Strukturu

```
nft/
│
├── mod.rs          # Əsas giriş nöqtəsi; bütün modulları eksport edir
├── token.rs        # NFT ledger əsas məntiqi: mint, burn, transfer, approval
├── metadata.rs     # NFT metadata idarəetməsi
├── collection.rs   # Kolleksiya məntiqi və NFT qruplaşdırılması
├── royalty.rs      # Royalty bölüşdürmə sistemi
├── marketplace.rs  # Marketplace list və satış məntiqi
├── auction.rs      # Auksion mexanizmi və bidding axını
├── governance.rs   # NFT əsaslı idarəetmə və DAO vasitələri
├── error.rs        # NFT-ə aid xətalar və nəticələr
├── events.rs       # NFT ilə bağlı chain event-lər
├── types.rs        # Paylaşılan NFT strukturları və enum-lar
├── storage.rs      # NFT-lər üçün storage layer
├── tests.rs        # Unit və inteqrasiya testləri
```

---

## İnteqrasiya

- `/identity` modulu ilə sıx inteqrasiya: SBT-lər, DAO rolları, və icazəli NFT-lər  
- `/core/consensus` ilə validator şəxsiyyəti və staking əsaslı NFT sistemləri  
- `/liquidity` və `/governance` modulları ilə DeFi və DAO fəaliyyətləri üçün uyğunlaşdırılmışdır

---

## Təhlükəsizlik və Uyğunluq

- ERC-721, ERC-1155 və ERC-2981 standartlarına tam uyğundur  
- Hadisə əsaslı, auditə açıq və DAO ilə idarə edilə bilən struktur  
- Marketplace-lər, airdrop-lar, DeFi vault-lar və DAO xəzinələri üçün təhlükəsiz dizayn edilmişdir

---

## Lisenziya

Bu modul OCOS-Chain protokolunun bir hissəsidir. İstifadə şərtləri üçün [LICENSE](../../LICENSE) sənədinə baxın.
