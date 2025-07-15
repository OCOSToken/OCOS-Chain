# OCOS-Chain: İdarəetmə (Governance) Smart Müqavilə Modulu

**Modulyar | Genişlənəbilən | Zəncir üstü Səsvermə | DAO Səviyyəsində Təhlükəsizlik**

---

## Ümumi Baxış

`contracts/governance` modulu OCOS-Chain üçün tam xüsusiyyətli, auditə açıq və genişlənə bilən idarəetmə sistemini təqdim edir. Burada şura əsaslı, ağırlıqlı, referendum və nümayəndəlik (delegation) səsvermə; təklifin bütün həyat dövrü; parametrlərin və konfiqurasiyanın idarə olunması; icra loqikası; tam hadisə və storage infrastrukturu — hamısı smart müqavilə qatında qurulub.

---

## Əsas Xüsusiyyətlər

- **Şura idarəetməsi:** Multisig komitələr, rotasiya üzvləri, emergency icra
- **Ağırlıqlı səsvermə:** Stake, reputasiya və ya token əsaslı voting gücü və limiti
- **Referendum:** DAO səviyyəsində binary, approval və quadratic voting ilə əsas dəyişikliklər
- **Delegasiya:** Likvid demokratiya və proxy səsvermə mexanizmləri
- **Təklifin həyat dövrü:** Yaradılma, səsvermə müddəti, status, qəbul/rədd, icra və audit log
- **Konfiqurasiya olunan parametrlər:** Zəncir üstü dəyişə bilən voting, quorum, threshold qaydaları
- **Hadisə əsaslı arxitektura:** Təklif, səs, icra, konfiqurasiya dəyişikliyi üçün tam event yayımı
- **Audit və upgrade dəstəyi:** Hər bir əməliyyat loglanır, registry ilə təhlükəsiz governance engine upgrade

---

## Qovluq Strukturu

```
contracts/governance/
│
├── mod.rs           # Əsas giriş nöqtəsi, bütün governance modullarını eksport edir
├── council.rs       # Şura əsaslı səsvermə məntiqi
├── weighted_vote.rs # Token, stake və ya reputasiya əsaslı səsvermə
├── referendum.rs    # Referendum və ümumi səsvermə məntiqi
├── delegation.rs    # Delegasiya (likvid demokratiya)
├── proposal.rs      # Təklif strukturu və həyat dövrü
├── config.rs        # İdarəetmə parametrləri və sazlamaları
├── execution.rs     # Təklif icrası məntiqi
├── registry.rs      # Governance engine-lərin dinamik qeydiyyatı
├── error.rs         # Error tipləri
├── types.rs         # Əsas enum və strukturlar
├── events.rs        # Bütün əməliyyatlar üçün event yayımı
├── storage.rs       # Təklif, voting və council üçün davamlı storage
├── tests.rs         # Governance contract unit və integration testləri
```

---

## Təhlükəsizlik və Audit

- **Rol əsaslı və threshold nəzarəti:** Hər zəncir üstü əməliyyat voting və qaydalar əsasında yoxlanır
- **Double-voting, reentrancy və replay qoruması:** Bütün əsas əməliyyatlar atomik və idempotentdir
- **Hadisə logu:** Hər bir təklif, səs, icra, delegasiya və parametr dəyişikliyi event olaraq yayımlanır
- **Özünü-amendment:** Governance engine registry və təklif mexanizmi ilə zəncir üzərində təhlükəsiz şəkildə yenilənə bilir

---

## İnteqrasiya

- `/core/consensus` ilə sıx inteqrasiya (blok təsdiqi hook-ları)
- `/crypto` modulunda imza və voting gücü yoxlaması
- `/ledger` ilə DAO və təklif vəziyyəti üçün oxu/yazma

---

## Lisenziya

Bu modul OCOS-Chain layihəsinin bir hissəsidir. Şərtlər üçün baxın: [LICENSE](../../LICENSE)
