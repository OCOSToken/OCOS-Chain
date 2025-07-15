# OCOS-Chain: DAO Smart Müqavilə Modulu

**Decentralizasiya | Modulyar | İdarəetmə əsaslı | Auditə Açıq**

---

## Ümumi Baxış

`contracts/dao` modulu OCOS-Chain üçün tam funksional zəncir üstü DAO (Decentralized Autonomous Organization) idarəetmə sistemini təmin edir. Burada təkliflərin idarəsi, çəkiyə əsaslanan səsvermə, üzvlük loqikası, treasury idarəetməsi və icmanın verdiyi qərarların təhlükəsiz icrası — bütün bunlar smart müqavilə səviyyəsində qurulub.

---

## Əsas Xüsusiyyətlər

- **Təklif dövrü:** Zəncir üstü təkliflərin yaradılması, baxışı, təsdiqlənməsi, növbəyə qoyulması, icrası və rədd edilməsi
- **Çəkiyə əsaslanan səsvermə:** Stake, reputasiya və ya ünvan əsaslı səsvermə gücü və quorum qaydaları
- **Üzvlük idarəsi:** Rollar, tier-lər, daxilolma/çıxma, dinamik icazə sistemi
- **Treasury modulu:** DAO fondlarının idarəsi, qrant ayrılması və təhlükəsiz vəsait çıxarılması
- **Konfiqurasiya oluna bilən idarəetmə:** Quorum, səsvermə müddəti, həddlər və icra gecikmələri istəyə görə tənzimlənir
- **Auditə açıq storage:** Təklif və səsvermə dəftəri, tam şəffaflıq və event log-ları
- **Zəncir üstü yenilənmə:** DAO özünün parametrlərini və kodunu yeniləyə bilər

---

## Qovluq Strukturu

```
contracts/dao/
│
├── mod.rs           # Əsas DAO müqaviləsinin giriş nöqtəsi
├── proposal.rs      # Təklif strukturu və idarəetmə loqikası
├── voting.rs        # Səsvermə mexanizmi (stake/reputasiya/rol əsaslı)
├── membership.rs    # DAO üzvlüyü və reputasiya sistemi
├── execution.rs     # Qəbul edilmiş təkliflərin icrası
├── config.rs        # İdarəetmə parametrləri (quorum, gecikmə və s.)
├── treasury.rs      # DAO vəsaitləri və qrant idarəetməsi
├── error.rs         # DAO üçün spesifik error tipləri
├── types.rs         # Əsas enum və strukturlar (ProposalStatus, VoteType və s.)
├── storage.rs       # Davamlı storage (təklif dəftəri, səsvermə jurnalı)
├── events.rs        # DAO hadisələrinin yayımı (indexing və tracing üçün)
├── tests.rs         # Unit və inteqrasiya testləri
```

---

## Təhlükəsizlik və Audit

- **Zəncir üstü icazə yoxlamaları:** Bütün dəyişiklik əməliyyatları rollar və həddlər əsasında yoxlanılır
- **Reentrancy və replay qoruması:** Təklif icrası atomik, səsvermə məntiqi isə idempotentdir
- **Tam izlenebilirlik:** Təklif, səs və icra hadisələri hər bir status dəyişikliyində yayımlanır
- **Yenilənəbilən idarəetmə:** DAO öz idarəetmə və kod loqikasını təklif və səsvermə ilə yeniləyə bilər

---

## İstifadə Ssenariləri

- DAO əsaslı treasury və qrant idarəetməsi
- Protokol yenilənməsi və on-chain parametr dəyişiklikləri
- İcma əsaslı şura, fond və ya siyasət dəyişikliyi üçün səsvermə
- NFT layihə DAO-ları, pul toplanan aktivlərin idarəsi, investisiya sindikatları

---

## İnteqrasiya

- `/core/consensus` ilə DAO hook-lar vasitəsilə blok validasiyasında işləyir
- `/crypto` modulu ilə səs verənin imzası və stake yoxlanışı
- `/ledger` ilə təklif və vəziyyət izlənməsi üçün inteqrasiya

---

## Lisenziya

Bu modul OCOS-Chain layihəsinin bir hissəsidir. Şərtlər üçün bax: [LICENSE](../../LICENSE)
