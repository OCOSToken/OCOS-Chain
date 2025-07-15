# OCOS-Chain: Virtual Maşın (VM) Alt Sistemi

**Modulyar | Deterministik | Gas Ölçülü | DAO-Ağıllı İcra Mühərriki**

---

## Ümumi Baxış

`/vm` modulu OCOS-Chain-də bütün smart müqavilələrin, DAO məntiqinin və idarəetmə təkliflərinin icra mühitini təmin edir. Bu modul modulyar, gas limitli və təhlükəsiz şəkildə hazırlanmışdır. WebAssembly (WASM) dəstəkləyir və gələcəkdə digər VM-lərlə (məsələn: MoveVM, EVM) genişlənə bilir.

---

## Xüsusiyyətlər

- **WASM smart müqavilə icrası** sandbox rejimində və gas nəzarəti altında
- **Gas ölçülməsi** və təhlükəli döngülərin qarşısının alınması (trap detection)
- **Syscall-lar** vasitəsilə yaddaşa, loglara və hadisə sisteminə çıxış
- **İcra mühiti** daxilində block məlumatları, istifadəçi, DAO metadata və s.
- **Nəticə emalı** və müxtəlif `ExitReason`, `VmError` növləri ilə idarəetmə
- **Çoxlu VM dəstəyi** — gələcək üçün dinamik VM registry mexanizmi

---

## Qovluq Strukturu

```
vm/
│
├── mod.rs         # VM komponentlərini birləşdirən əsas modul
├── engine.rs      # İcra mühərriki və VM interfeysi
├── wasm.rs        # WebAssembly dəstəyi
├── context.rs     # Caller, block, DAO və s. məlumatları
├── memory.rs      # Təhlükəsiz virtual yaddaş modeli
├── gas.rs         # Gas istifadəsi və limiti idarəsi
├── syscall.rs     # Host funksiyalar: log, storage, call və s.
├── result.rs      # VM nəticə strukturları (VmResult, VmError)
├── registry.rs    # Müxtəlif VM-lərin qeydiyyatı və yönləndirilməsi
├── tests.rs       # Testlər və audit ssenariləri
```

---

## İstifadə Ssenariləri

- İdarəetmə və maliyyə modullarında smart müqavilələrin icrası
- DAO səviyyəsində səsvermə təkliflərinin zəncir üzərində tətbiqi
- NFT əməliyyatları, aktivlərin yaradılması və bridge doğrulama
- Stake və treasury qaydaları daxilində proqramlaşdırıla bilən icra

---

## Təhlükəsizlik Nəzərəçarpanlar

- **Gas ölçülməsi** ilə DoS və sonsuz döngülərin qarşısı alınır
- **Syscall-lar** yalnız icazəli (whitelist) funksiyalarla məhdudlaşdırılır
- **Trap halları** nəticə olaraq təhlükəsiz revert ilə nəticələnir
- **Yaddaş** hər kontekstdə təcrid olunmuş şəkildə idarə olunur

---

## Gələcək Genişlənmələr

- EVM və MoveVM dəstəyi üçün `VmEngine` interfeysi vasitəsilə genişlənmə
- zkVM inteqrasiyası (zero-knowledge proof əsaslı smart müqavilələr)
- DAO vasitəsilə yeni VM mühərriklərinin zəncir üzərindən aktivləşdirilməsi

---

## Müəlliflər və İdarəetmə

- OCOS Foundation — [https://ocos.io](https://ocos.io)
- Protokol rəhbəri: Ocoshy Nakomoto — [https://github.com/Ocoshy](https://github.com/Ocoshy)

---

## Lisenziya

Bu modul OCOS-Chain layihəsinin bir hissəsidir. İstifadə şərtləri üçün baxın: [LICENSE](../LICENSE)
