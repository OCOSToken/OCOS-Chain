# OCOS-Chain Konsensus Layihəsi

**Hibrid | Kvant Dayanıqlı | DAO-Əsaslı | Auditə Açıq**

---

## Ümumi Baxış

`/core/consensus` modulu OCOS-Chain protokolunun əsas nüvəsidir. Blokların təhlükəsiz yaradılması, yoxlanılması və on-chain idarəetməni həyata keçirir. Bu qat həm **klassik**, həm də **post-quantum kriptoqrafiyasını** birləşdirərək gələcək üçün dayanıqlı və təhlükəsiz konsensus təmin edir. Modullar genişlənə bilən, auditə uyğun və yüksək performanslı şəkildə hazırlanıb.

---

## Əsas Xüsusiyyətlər

- **Hibrid Konsensus Mühərriki**  
  Proof-of-Stake (PoS), Proof-of-Authority (PoA) və hibrid rejimləri dəstəkləyir. DAO vasitəsilə idarə olunur.

- **Kvant Dayanıqlı İmzalar**  
  Dilithium, XMSS kimi post-quantum imzalar və klassik Ed25519 dəstəyi ilə təhlükəsiz imzalama strukturu.

- **Validator İdarəetməsi**  
  Stake etmə, slashing, jail/unjail, çıxarılma və metadata idarəsi.

- **Blok və Başlıq Məntiqi**  
  Hashing, dövlət və əməliyyat kökləri ilə blok strukturu. Metadata üçün genişləndirilə bilən başlıq sahəsi.

- **DAO On-Chain İdarəetməsi**  
  Təklif və səsvermə sistemi. Konsensus parametrlərinin dəyişməsi tam audit qeydləri ilə icra olunur.

- **Tam Test Əhatəsi**  
  Validator davranışı, imzalama (klassik və quantum), idarəetmə və konsensus ssenariləri üçün avtomatlaşdırılmış testlər.

---

## Qovluq Strukturu

```
/core/consensus
│
├── mod.rs               # Konsensus modulunun əsas kökü, bütün API-ləri açıqlayır
├── consensus_engine.rs  # Konsensus loqikası, lider seçimi, blok təklifi və təsdiqləmə
├── validator.rs         # Validatorların identifikasiyası və idarə olunması
├── quantum_sig.rs       # Klassik və kvant imzalar üçün interfeys
├── block.rs             # Blok və başlıq strukturu, hash və imza yoxlaması
├── slashing.rs          # Validator səhvlərinə görə slashing mexanizmi
├── governance.rs        # DAO təklif və idarəetmə sistemləri
├── tests.rs             # Unit və inteqrasiya testləri
```

---

## Təhlükəsizlik və Yaxşı Praktikalar

- **Kvant təhlükəsizliyi** – Dilithium, XMSS və digər alqoritmləri dəstəkləyir.
- **DAO-əsaslı idarəetmə** – Konsensus parametrləri təklif və səsvermə yolu ilə dəyişdirilə bilər.
- **Audit uyğunluğu** – Hər bir hadisə izlənə və yoxlanıla bilər.
- **Modularlıq** – Asan genişləndirilə bilən və dəyişə bilən arxitektura.

---

## Genişlənmə İmkanları

- Yeni konsensus rejimləri əlavə etmək (DPoS, BFT və s.)
- `quantum_sig.rs` üzərindən kriptografik metodların artırılması
- DAO ssenarilərinin on-chain və off-chain idarəetmə ilə inteqrasiyası
- Audit qeydlərinin və uyğunsuzluq hesabatlarının təkmilləşdirilməsi

---

## İstinadlar

- [Whitepaper](../../docs/WHITEPAPER.md)
- [Protokollar](../../docs/PROTOCOLS.md)
- [Təhlükəsizlik Siyasəti](../../docs/SECURITY.md)
- [API Arayışı](../../docs/API.md)
- [İdarəetmə Modeli](../../docs/GOVERNANCE.md)

---

## Müəlliflər və Baxıcılar

- [OCOS Foundation](https://ocos.io)
- [Ocoshy Nakomoto](https://github.com/Ocoshy)

---

## Lisenziya

Bu modul OCOS-Chain açıq mənbə layihəsinin bir hissəsidir. Ətraflı məlumat üçün bax: [LICENSE](../../LICENSE)
