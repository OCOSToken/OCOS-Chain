# OCOS-Chain: Şəbəkə (Network) Altsistemi

**P2P | Modular | Auditable | Scalable | Blockchain-Ready**

---

## Ümumi Baxış

`/network` qovluğu OCOS-Chain protokolunun şəbəkə səviyyəli bütün funksiyalarının əsasını təşkil edir. Burada **P2P rabitə**, **peer identifikasiyası**, **mesajlaşma**, **sinxronizasiya**, **node statusu**, **data ötürülməsi** və **audit** imkanları yer alır.  
Modul **modular**, **gələcəyə dözümlü** və **təhlükəsiz** qurulub.

---

## Əsas Altmodullar və Fayllar

| Fayl/Qovluq       | Funksiya və Məqsəd                                                        |
|-------------------|----------------------------------------------------------------------------|
| `mod.rs`          | Şəbəkə modulunun əsas giriş nöqtəsi; bütün alt-modulları birləşdirir        |
| `p2p.rs`          | Peer-to-peer rabitə, discovery, ban list, status və əlaqələrin idarəsi      |
| `transport.rs`    | TCP/UDP/WebSocket, TLS, QUIC və s. üçün universal rabitə layer-i           |
| `message.rs`      | Blok, tx, ping, gossip və s. mesaj tiplərinin standart strukturlaşdırılması |
| `codec.rs`        | Mesajların serialization/deserialization (bincode, JSON, protobuf və s.)    |
| `node.rs`         | Node identifikasiyası, status izlənməsi, agent və etibar idarəsi           |
| `sync.rs`         | Blok və tx sinxronizasiya, snapshot və state fərqlərinin paylaşılması       |
| `tests.rs`        | Şəbəkə komponentləri üçün unit və integration testləri                     |

---

## Əsas Xüsusiyyətlər

- **Tam P2P şəbəkə** – Peer discovery, ban/whitelist, handshake və bağlantıların effektiv idarəsi.
- **Multi-protokol rabitə** – TCP, WebSocket, TLS və s. layer-lar üçün universal interfeys.
- **Auditə açıq mesajlaşma** – Bütün mesajlar serialization və tracing üçün standartlaşdırılıb.
- **Sinxronizasiya və data paylaşımı** – Fork-un qarşısının alınması, snapshot və state diff-lərin effektiv ötürülməsi.
- **Node identifikasiyası və trust score** – Governance və ağıllı şəbəkə üçün reputasiya və monitorinq.

---

## Qovluq Strukturu

```
network/
│
├── mod.rs
├── p2p.rs
├── transport.rs
├── message.rs
├── codec.rs
├── node.rs
├── sync.rs
└── tests.rs
```

---

## Təhlükəsizlik və Yaxşı Praktikalar

- **DoS və Sybil qoruması:** Ban siyahısı, heartbeat izlənməsi, rate-limiting və reputasiya sistemi ilə.
- **Mesajların auditə açıq saxlanması:** UUID, timestamp və log formatları.
- **Layer-agnostic protokol:** Yeni protokolları əlavə etmək və mövcud layer-ları dəyişmək asandır.

---

## İstifadə Ssenariləri

- Yeni node-ların şəbəkəyə qoşulması və sinxronizasiyası
- Blok və əməliyyatların yayılması
- DAO snapshot və governance sinxronizasiyası
- Real-time ping/pong, node monitorinq və şəbəkə statistikası
- Multi-protokol data ötürülməsi və şəbəkə testləri

---

## Əlaqə və Dəstək

- [OCOS Foundation](https://ocos.io)
- [Ocoshy Nakomoto (Şəbəkə Arxitektoru)](https://github.com/Ocoshy)

---

## Lisenziya

Bu modul OCOS-Chain layihəsinin bir hissəsidir. Ətraflı məlumat üçün bax: [LICENSE](../LICENSE)
