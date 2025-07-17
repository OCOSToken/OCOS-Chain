# OCOS-Chain: Audit, Uyğunluq və Forensik Layer

**İz (Trace) | Jurnal (Log) | Uyğunluq (Compliance) | Hadisə (Event) | Sübut (Proof) | Analitika | Forensika**

---

## Ümumi Baxış

`/audit` modulu OCOS-Chain üçün tam modulyar və audita uyğun infrastruktur təqdim edir.  
Bu modul tracing (əməliyyat izi), strukturlu log, uyğunluq yoxlamaları, event normalizasiyası, sübut/commitment yaradılması, indeksasiya, analitika, yaddaş (storage) və səhv (error) idarəsi kimi funksiyaları təmin edir.  
Həm on-chain, həm də off-chain audit və tənzimləyici (regulator) tələblərə cavab verir.

---

## Xüsusiyyətlər

- **Tam iz və forensika:** Əməliyyat, governance, VM, storage və DAO səviyyəsində izləmə
- **Strukturlu jurnal:** İstifadəçi, zaman, əməliyyat və nəticə ilə audit log-lar
- **Uyğunluq yoxlamaları:** KYC, DAO üzvlüyü, treasury çıxarışları və səsvermə gücü
- **Hadisə normalizasiyası:** Müxtəlif contract-lardan gələn event-lərin eyniləşdirilməsi və dekodlanması
- **Merkle/commitment/zk-proof dəstəyi:** Dəyişməz sübutlar və zero-knowledge genişlənməsi
- **İndeksasiya və analitika:** Bütün audit məlumatları üçün yüksək sürətli sorğu, API və dashboard dəstəyi
- **Performans göstəriciləri:** DAO və protokol üçün real-time və tarixi metriklər, risk analizi
- **Universal yaddaş:** Bütün log, event, sübut və uyğunluq qeydləri üçün vahid audit storage sistemi

---

## Qovluq Strukturu

```
audit/
│
├── mod.rs         # Əsas modul; bütün audit alt-modullarını birləşdirir
├── trace.rs       # Əməliyyat, governance və DAO üçün tam izləmə mexanizmi
├── log.rs         # Strukturlu giriş və fəaliyyət log sistemi
├── event.rs       # Hadisələrin dekodlanması və normalizasiyası
├── compliance.rs  # KYC və DAO uyğunluq yoxlamaları
├── hash.rs        # Merkle kökləri və kriptoloji hash-lər
├── proof.rs       # Sübut strukturları (Merkle, commitment, zk-proof)
├── index.rs       # Analitik və axtarış interfeysi üçün indeksasiya
├── metrics.rs     # Protokol, DAO və VM performans göstəriciləri
├── types.rs       # Birləşmiş tip və enum strukturları
├── storage.rs     # Log, trace, event və sübutların yaddaşı
├── error.rs       # Audit sisteminə aid səhvlər
├── tests.rs       # Audit sistemi üçün tam test dəsti
```

---

## Təhlükəsizlik və Uyğunluq

- **Dəyişməz sübutlar:** Bütün audit izləri, log-lar və event-lər hash və commitment ilə qorunur
- **Tam uyğunluq yoxlaması:** KYC, DAO üzvlüyü və governance qaydaları zəncir üstündə tətbiq olunur
- **Forensik audit:** Hüquqi və regulator yoxlama üçün bütün trace və sübut qeydləri mövcuddur
- **Analitika inteqrasiyası:** Xarici API, dashboard və risk analiz alətləri ilə uyğunluq

---

## İnteqrasiya

- `/core/consensus`, `/governance`, `/identity`, `/liquidity` və VM modulları ilə tam inteqrasiya
- Xarici audit və risk idarəetmə sistemləri üçün audit API-ları mövcuddur

---

## Lisenziya

Bu modul OCOS-Chain layihəsinin bir hissəsidir. İstifadə şərtləri üçün baxın: [LICENSE](../LICENSE)
