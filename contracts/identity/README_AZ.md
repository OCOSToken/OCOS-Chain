# OCOS-Chain: On-Chain Şəxsiyyət və Reputasiya Modulu

**DID | Profil | Soulbound | KYC/KYB | Reputasiya | Bərpa | Qrup | Attestasiya | İdarəetmə**

---

## Ümumi Baxış

`/identity` modulu OCOS-Chain üçün tam modulyar, auditə hazır və genişlənə bilən şəxsiyyət infrastrukturu təqdim edir.  
Bu modul decentralized identifier-lər (DID), istifadəçi profilləri, soulbound token-lər (SBT), zəncir üzərində KYC/KYB yoxlamaları, reputasiya xalları, sosial və guardian əsaslı bərpa, attestasiya, qrup və rol üzvlüyü, habelə idarəetmə funksiyalarını dəstəkləyir.

---

## Xüsusiyyətlər

- **DID (Decentralized Identifier):** W3C uyğun on-chain identifikatorlar, açar idarəçiliyi və xidmət nöqtələri  
- **Profil idarəetməsi:** Zəncir üzərində istifadəçi adı, avatar, email və xüsusi sahələr  
- **Soulbound Token-lər (SBT):** KYC, DAO üzvlüyü və s. üçün köçürülə bilməyən etimad nişanları  
- **On-chain KYC/KYB:** İstifadəçi və təşkilat yoxlamaları, uyğunluq və ZK məxfiliyi ilə inteqrasiya  
- **Reputasiya sistemi:** Etibar xalları, DAO səsvermə, peer review və sosial sübutlar  
- **Guardian/sosial bərpa:** Seed, multisig, sosial və DAO əsaslı bərpa mexanizmləri  
- **Qrup üzvlüyü:** DAO, whitelist, rol, icma və airdrop əsaslı idarəetmə  
- **Attestasiya registri:** DAO və ya provayder tərəfindən verilən yoxlanıla bilən attestasiyalar  
- **İdarəetmə:** Parametr/protokol dəyişiklikləri, sosial bərpa təsdiqi, konfiqurasiya yeniləmələri  
- **Hadisə əsaslı arxitektura:** Bütün dəyişikliklər və əməliyyatlar chain-də event-lərlə izlənir  

---

## Qovluq Strukturu

```
identity/
│
├── mod.rs         # Əsas giriş nöqtəsi; bütün alt-modulları eksport edir
├── did.rs         # W3C uyğun DID məntiqi
├── profile.rs     # Profil idarəetməsi
├── soulbound.rs   # Soulbound token-lər və etimad sertifikatları
├── kyc.rs         # On-chain KYC/KYB və uyğunluq
├── reputation.rs  # Etibar/reputasiya sistemi
├── recovery.rs    # Sosial və guardian əsaslı bərpa
├── attestation.rs # DAO/provayder attestasiya sistemləri
├── group.rs       # Qrup/rol/DAO üzvlüyü və whitelist mexanizmləri
├── governance.rs  # Identity əsaslı təklif və səsvermə sistemi
├── registry.rs    # DID, profil, sbt və s. üçün mərkəzləşdirilmiş sorğu sistemi
├── zkid.rs        # Zero-knowledge identity doğrulama
├── privacy.rs     # Məxfilik və uyğunluq utilitləri (ZK, mixer və s.)
├── error.rs       # Xəta tipləri və nəticələr
├── events.rs      # Bütün chain event-ləri üçün strukturlar
├── types.rs       # Paylaşılan tiplər, enum-lar və strukturlar
├── storage.rs     # Bütün modullar üçün daimi yaddaş
├── tests.rs       # Unit və inteqrasiya testləri
```

---

## Təhlükəsizlik və Audit

- **Hər dəyişiklik üçün DAO və on-chain idarəetmə**
- **Multisig və sosial/guardian əsaslı bərpa**
- **KYC, SBT, identitet və governance əməliyyatları üçün tam audit izi**
- **Zero-knowledge və hash əsaslı məxfilik dəstəyi**

---

## İnteqrasiya

- `/core/consensus` ilə validator, staking və üzvlük yoxlamaları üçün işləyir  
- `/nft`, `/liquidity`, `/governance`, və `/dao` modulları ilə icazə və rol idarəetməsi üçün inteqrasiya olunur  
- Web3 tətbiqləri, DAO-lar, DeFi və müəssisələr üçün təhlükəsiz, genişlənə bilən identitet infrastrukturunu təmin edir  

---

## Lisenziya

Bu modul OCOS-Chain layihəsinin bir hissəsidir. İstifadə şərtləri üçün [LICENSE](../../LICENSE) sənədinə baxın.
