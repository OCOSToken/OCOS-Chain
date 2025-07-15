# OCOS-Chain: Kriptografiya Altsistemi (`/crypto`)

**Quantum-Resistant | Modular | Auditable | Blockchain-Grade Security**

---

## Ümumi Baxış

`/crypto` qovluğu OCOS-Chain platformasının bütün təhlükəsizlik, identifikasiya, imzalama və açar idarəetmə mexanizmlərinin təməlini təşkil edir. Burada həm **klassik** (Ed25519, secp256k1, AES-GCM), həm də **kvant dayanıqlı** (Dilithium, XMSS, Kyber, Falcon) alqoritmlər yer alır. Bütün modullar **modular**, **auditə uyğun** və **asan genişlənə bilən** arxitektura üzərində qurulub.

---

## Əsas Modullar və Altqovluqlar

| Qovluq/Fayl             | Məqsəd və Funksiya                                                        |
|------------------------ |----------------------------------------------------------------------------|
| `/classical`            | Klassik kriptografiya (Ed25519, secp256k1, AES-GCM və s.)                 |
| `/quantum`              | Post-quantum (NIST PQC finalistləri: Dilithium, Kyber, XMSS, Falcon və s.)|
| `hashing.rs`            | SHA3, BLAKE3 və s. universal hash funksiyaları                            |
| `kdf.rs`                | Açar törətmə funksiyaları (PBKDF2, Argon2, scrypt)                        |
| `utils.rs`              | Universal utilitlər: random, encoding, zeroize və s.                      |

---

## Əsas Xüsusiyyətlər

- **Modular arxitektura:** Klassik və quantum kriptografiya, hashing, utilitlər və KDF vahid interfeys altında.
- **Auditə açıq:** Hər bir funksiya və modul audit və test üçün xüsusi hazırlanıb.
- **Gələcəyə dözümlü:** Quantum-safe (Dilithium, Kyber, XMSS, Falcon) və sənaye standartı (Ed25519, secp256k1, AES) birlikdə.
- **Universal istifadə:** Core, consensus, wallet, DAO, bridge, airdrop, confidential messaging və s. üçün uyğun.

---

## Qovluq Strukturu

```
crypto/
│
├── mod.rs
│
├── classical/
│   ├── mod.rs
│   ├── ed25519.rs
│   ├── secp256k1.rs
│   └── aes_gcm.rs
│
├── quantum/
│   ├── mod.rs
│   ├── dilithium.rs
│   ├── kyber.rs
│   ├── xmss.rs
│   └── falcon.rs
│
├── hashing.rs
├── kdf.rs
├── utils.rs
```

---

## Təhlükəsizlik Standartları və Tövsiyələr

- **NIST PQC finalistləri**: Dilithium, Kyber, Falcon, XMSS – uzunmüddətli blockchain təhlükəsizliyi üçün əsas seçimlərdir.
- **Klassik blokçeyn standartları**: Ed25519 (Solana, Monero), secp256k1 (Bitcoin, Ethereum), AES-GCM.
- **Salt, random və zeroize**: Açar və həssas məlumatların idarə edilməsində həmişə CSPRNG və sıfırlama tətbiq olunur.
- **Test və audit**: Hər bir modul üçün unit və integration testləri, audit izləri yaradılıb.

---

## İstifadə Ssenariləri

- **Blok imzası və yoxlanması**
- **DAO təklifləri və səsvermə identifikasiyası**
- **Validator autentifikasiyası və stake prosesləri**
- **Açar törətmə və wallet yaradılması**
- **Şəxsi məlumatların şifrələnməsi və confidential mesajlaşma**
- **Address və hash əsaslı identifikasiya**

---

## Əlaqə və Dəstək

- [OCOS Foundation](https://ocos.io)
- [Ocoshy Nakomoto (Protokol Arxitektoru)](https://github.com/Ocoshy)

---

## Lisenziya

Bu modul OCOS-Chain layihəsinin bir hissəsidir. Ətraflı məlumat üçün bax: [LICENSE](../LICENSE)
