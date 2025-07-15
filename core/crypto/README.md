# OCOS-Chain: Cryptography Subsystem (`/crypto`)

**Quantum-Resistant | Modular | Auditable | Blockchain-Grade Security**

---

## Overview

The `/crypto` directory forms the foundation of OCOS-Chain's security, identity, signature, and key management mechanisms. It includes both **classical** (Ed25519, secp256k1, AES-GCM) and **post-quantum** (Dilithium, XMSS, Kyber, Falcon) cryptographic algorithms. All modules are designed with **modularity**, **auditability**, and **easy extensibility** in mind.

---

## Main Modules and Subdirectories

| Folder/File             | Purpose and Function                                                           |
|-------------------------|--------------------------------------------------------------------------------|
| `/classical`            | Classical cryptography (Ed25519, secp256k1, AES-GCM, etc.)                    |
| `/quantum`              | Post-quantum cryptography (NIST PQC finalists: Dilithium, Kyber, XMSS, Falcon)|
| `hashing.rs`            | Universal hash functions (SHA3, BLAKE3, etc.)                                  |
| `kdf.rs`                | Key derivation functions (PBKDF2, Argon2, scrypt)                              |
| `utils.rs`              | Universal utilities: random, encoding, zeroize, and more                       |

---

## Key Features

- **Modular architecture:** Unified interfaces for classical and quantum crypto, hashing, utilities, and KDFs.
- **Auditable:** Every function and module is testable and designed with audit in mind.
- **Future-proof:** Combines Quantum-safe (Dilithium, Kyber, XMSS, Falcon) and industry-standard (Ed25519, secp256k1, AES) primitives.
- **Universal usage:** Applicable to core, consensus, wallet, DAO, bridge, airdrop, confidential messaging, and more.

---

## Directory Structure

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

## Security Standards & Recommendations

- **NIST PQC finalists:** Dilithium, Kyber, Falcon, XMSS are top-tier long-term secure options.
- **Classical blockchain standards:** Ed25519 (Solana, Monero), secp256k1 (Bitcoin, Ethereum), AES-GCM.
- **Salt, random, and zeroize:** Proper use of CSPRNG and memory clearing for sensitive data.
- **Testing & audit:** Each module includes unit/integration tests and traceable audit points.

---

## Use Cases

- **Block signing and verification**
- **DAO proposal and vote authentication**
- **Validator identity and staking**
- **Wallet generation and key derivation**
- **Encrypted storage and confidential messaging**
- **Address and hash-based identification**

---

## Contact & Support

- [OCOS Foundation](https://ocos.io)
- [Ocoshy Nakomoto (Protocol Architect)](https://github.com/Ocoshy)

---

## License

This module is part of the OCOS-Chain project. See: [LICENSE](../LICENSE) for details.
