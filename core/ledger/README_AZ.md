# OCOS-Chain: Ledger Alt Sistemi (`/ledger`)

**Vəziyyətə əsaslanan | Auditə açıq | Modul əsaslı | Blockchain-Mühasibat**

---

## Ümumi Baxış

`/ledger` qovluğu OCOS-Chain blokçeyninin daimi vəziyyət infrastrukturunun əsasını təşkil edir. Bu qovluq bütün blokları, əməliyyatları, hesab balanslarını, vəziyyət dəyişikliklərini və tarixə dair isbatları qeyd edir.  
Yüksək auditə uyğunluq, modul inteqrasiyası və yüksək performanslı deterministik icra üçün nəzərdə tutulub.

---

## Əsas Modullar və Fayllar

| Fayl / Qovluq        | Məqsəd və Funksiya                                                            |
|----------------------|-------------------------------------------------------------------------------|
| `mod.rs`             | Ledger əsas modulu; bütün ledger komponentlərini birləşdirir                  |
| `block.rs`           | Blok strukturu, başlıq və blok doğrulama məntiqi                             |
| `transaction.rs`     | Əməliyyatlar, imzalanmış əməliyyatlar və əsas emal qaydaları                 |
| `state.rs`           | Vəziyyət idarəetməsi: balanslar, nonce-lar, kontrakt kodu və storage root-lar|
| `block_store.rs`     | Bütün blokların daimi saxlanması və indeksli əldə olunması                   |
| `tx_pool.rs`         | Təsdiqlənməmiş / gözləyən əməliyyatlar üçün mempool                          |
| `executor.rs`        | Deterministik blok və əməliyyat icra mühərriki                              |
| `receipt.rs`         | Əməliyyat qəbzləri və audit/tracing üçün event log-lar                       |
| `snapshot.rs`        | Snapshot / bərpa mexanizmi: backup, sürətli sinxronizasiya, fork dəstəyi     |
| `history.rs`         | Blok və əməliyyat tarixçəsi; explorer və geri qaytarma (rewind) üçün dəstək |
| `audit.rs`           | Ledger audit mexanizmi, izləmə və uyğunluq üçün log-lar                      |
| `tests.rs`           | Bütün ledger modulları üçün unit və inteqrasiya testləri                     |

---

## Əsas Xüsusiyyətlər

- **Vəziyyət əsaslı mühasibat:** Hər blok, əməliyyat, hesab və kontrakt vəziyyəti izlənir
- **Auditə uyğun dizayn:** Bütün əməliyyatlar üçün qəbzlər, tarixçə və isbatlar saxlanılır
- **Modul əsaslı:** Hər modul ayrılıqda dəyişdirilə və ya təkmilləşdirilə bilər
- **Sürətli sinx və backup:** Snapshot/bərpa mexanizmləri ilə yeni node-lar və fork üçün əlverişli
- **Explorer dəstəyi:** API dostu interfeys: blok, əməliyyat, event log, tarix axtarışı və analiz üçün

---

## Qovluq Strukturu

```
ledger/
│
├── mod.rs
├── block.rs
├── transaction.rs
├── state.rs
├── block_store.rs
├── tx_pool.rs
├── executor.rs
├── receipt.rs
├── snapshot.rs
├── history.rs
├── audit.rs
└── tests.rs
```

---

## İstifadə Ssenariləri

- Blok və əməliyyatların yadda saxlanması və geri çağırılması
- On-chain balans, nonce və kontrakt storage idarəetməsi
- Audit, uyğunluq və event tracing
- Snapshot ilə node sinxronizasiyası və fork bərpası
- DApp və DAO explorer dəstəyi (əməliyyat/event axtarışı, state isbatları və s.)

---

## Layihə rəhbərləri və Əlaqə

- [OCOS Foundation](https://ocos.io)
- [Ocoshy Nakomoto (Protokol Arxitektoru)](https://github.com/Ocoshy)

---

## Lisenziya

Bu modul OCOS-Chain açıq mənbə layihəsinin bir hissəsidir.  
Ətraflı məlumat üçün bax: [LICENSE](../LICENSE)
