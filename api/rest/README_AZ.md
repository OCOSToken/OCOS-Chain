# OCOS-Chain: REST API Modulu

**Modulyar | Auditə Uyğun | Təhlükəsiz | Enterprise Səviyyəli**

---

## Ümumi Baxış

`/api/rest` modulu OCOS-Chain üçün bütün zəncir məlumatlarının, əməliyyatlarının və idarəetmə sistemlərinin HTTP/JSON vasitəsilə əsas giriş nöqtəsidir.  
Bu modul explorer-lər, cüzdanlar, dApp-lar, DAO panelləri, birjalar, analitika və üçüncü tərəf xidmətlər üçün tam auditə uyğun, genişlənə bilən və production səviyyəli REST API təqdim edir.

---

## Xüsusiyyətlər

- **Birləşdirilmiş endpoint router:** Bloklar, əməliyyatlar, hesablar, governance, identity, liquidity, DAO, analitika
- **OpenAPI/Swagger sənədləşməsi:** İnteraktiv API explorer və avtomatik kod generasiyası
- **Token/JWT əsaslı autentifikasiya:** Public və qorunan endpoint-lər üçün təhlükəsiz, rol əsaslı giriş
- **Standart error sistemi:** Bütün xətalar HTTP status kodları və JSON strukturu ilə qaytarılır
- **CORS, gzip, loglama, rate-limit və monitorinq:** Təhlükəsizlik və performans üçün ən yaxşı təcrübələr
- **API versiyalaşdırma:** Geri uyğunluq dəstəyi ilə v1, v2 və s.
- **Avtomatik test sistemi:** CI/CD inteqrasiyası üçün tam endpoint testləri

---

## Qovluq Strukturu

```
api/rest/
│
├── mod.rs          # Əsas giriş nöqtəsi, bütün route və middleware-ləri birləşdirir
├── routes.rs       # Bütün HTTP yollarını yönləndirən router
├── handlers/       # API alt bölmələri üçün iş məntiqi:
│    ├── ledger.rs      # Blok, əməliyyat və vəziyyət endpoint-ləri
│    ├── governance.rs  # DAO, təkliflər, voting, şura və s.
│    ├── identity.rs    # DID, soulbound, KYC, reputasiya və profil
│    ├── liquidity.rs   # Pool-lar, swap, staking, oracle, bridge
│    ├── dao.rs         # DAO metadata, üzvlər, icazələr
│    ├── metrics.rs     # Performans və analitika statistikası
├── types.rs        # Ümumi cavab və sorğu tipləri (pagination, error və s.)
├── auth.rs         # Auth token və sessiya qoruma qatları
├── error.rs        # HTTP error mapping və cavab strukturları
├── version.rs      # API versiyası, build və sağlamlıq yoxlaması
├── docs.rs         # Swagger/OpenAPI sənədlərinin verilməsi
├── middleware.rs   # CORS, loglama, sıxılma, rate-limit və izləyişlər
├── tests.rs        # Bütün endpoint-lər üçün integration və test coverage
```

---

## Təhlükəsizlik və Audit

- **JWT və API açarı dəstəyi:** İcazəli və qorunan sorğular üçün token-based giriş
- **Rate-limiting:** Abuse qarşısının alınması
- **Sorğu/cavab loglaması:** Audit, təhlükəsizlik və forensic üçün
- **CI/CD-də test olunur:** Hər dəyişiklik tam endpoint testi ilə yoxlanılır

---

## İstifadə

- Web/mobil UI-lər, birjalar, oracle-lar, analitika panelləri və bot müştərilər üçün istifadəyə hazırdır
- Bütün endpoint-lər `/docs` vasitəsilə sənədləşdirilmişdir
- Public və admin səviyyəli API girişlərini dəstəkləyir

---

## Lisenziya

Bu modul OCOS-Chain layihəsinin bir hissəsidir. Bax: [LICENSE](../../LICENSE)
