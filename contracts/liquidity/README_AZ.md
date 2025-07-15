# OCOS-Chain: Likvidlik, AMM və DeFi Modulu

**Pool-lar | AMM | Staking | Farming | Oracle-lar | Bridge | Zəncir üstü İdarəetmə**

---

## Ümumi Baxış

`contracts/liquidity` modulu OCOS-Chain üçün DeFi və DEX protokolunun tam funksional qatını təqdim edir.  
Bura modular, auditə uyğun və genişlənə bilən infrastruktur — likvidlik pool-ları, AMM-lər, staking, farming, oracle-lar, cross-chain bridge-lər, rewards və governance — hamısı tam zəncir üstü həll kimi daxildir.

---

## Əsas Xüsusiyyətlər

- **Likvidlik pool-ları:** Token cütləri üçün LP token, depozit/çıxarış məntiqi
- **AMM swaps:** Avtomatlaşdırılmış market maker (Uniswap kimi) üçün swap, qiymət və invariant yoxlaması
- **Staking & farming:** Likvidlik təminatçıları üçün real-time reward və farming kampaniyaları
- **Oracle feeds:** Təhlükəsiz və yenilənə bilən qiymət və məlumat feed-ləri
- **Bridge:** Aktivlərin digər zəncirlərə lock, mint, burn və unlock ilə transferi, oracle təsdiqi ilə
- **Parametrlərin DAO-əsaslı tənzimlənməsi:** Bütün fee, limit, reward və intervallar DAO tərəfindən idarə olunur
- **Governance:** Likvidlik və DEX protokol dəyişikliyi üçün tam on-chain təklif, voting və icra
- **Event & audit:** Hər əməliyyat event kimi zəncir üstündə yayımlanır, tam izlənə və audit oluna bilir

---

## Qovluq Strukturu

```
contracts/liquidity/
│
├── mod.rs         # Əsas modul, bütün likvidlik bölmələrini birləşdirir
├── pool.rs        # Pool strukturu və məntiqi
├── amm.rs         # AMM swap və qiymət məntiqi
├── staking.rs     # Staking məntiqi (əgər ayrıca moduldursa)
├── farming.rs     # Farming və reward kampaniyası
├── rewards.rs     # Mükafatların bölüşdürülməsi
├── oracle.rs      # Oracle feed qeydiyyatı və yenilənməsi
├── bridge.rs      # Cross-chain asset bridge
├── config.rs      # Protokol parametrləri və DAO inteqrasiyası
├── governance.rs  # Likvidlik və DEX üçün DAO modulu
├── fee.rs         # Fee hesablanması və uçotu
├── router.rs      # Swap/trade/farming router məntiqi
├── types.rs       # Ümumi növlər və identifikatorlar
├── events.rs      # Bütün zəncir üstü event strukturları
├── error.rs       # Error kodları və mesajlar
├── storage.rs     # Bütün bölmələr üçün davamlı storage
├── tests.rs       # Unit və integration testləri
```

---

## Təhlükəsizlik və Audit

- **Slippage, fee və invariant yoxlamaları** hər pool və swap üçün tətbiq olunur
- **Rol və DAO-əsaslı dəyişiklik nəzarəti**
- **Event-driven monitoring:** Bütün dəyişikliklər event kimi zəncirdə qeyd olunur
- **On-chain təklif və upgrade ilə protokolun inkişafı**

---

## İnteqrasiya

- `/core/consensus` ilə blok təsdiqi və parametr dəyişikliyi üçün işləyir
- `/crypto` modulunda asset/account, reward və oracle imza yoxlanışı
- `/ledger` ilə balans və event-lərin zəncir üstü qeydi

---

## Lisenziya

Bu modul OCOS-Chain layihəsinin bir hissəsidir. İstifadə şərtləri üçün bax: [LICENSE](../../LICENSE)
