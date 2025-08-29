#!/usr/bin/env bash
set -euo pipefail

# 1) Qovluqlar
mkdir -p non_evm/solana tools/solana dashboard_patch

# 2) Solana/non-EVM rezerv ünvanları (Sənin verdiyin 7 ünvan, təkrarsız)
cat > non_evm/solana/RESERVE_ADDRESSES_SOLANA.json <<'JSON'
{
  "network": "bitcoin-mainnet",
  "as_of": "2025-08-29T00:00:00Z",
  "chain": "Solana-like (non-EVM gateway use)",
  "addresses": [
    { "label": "SOL-Reserve-001", "address": "3GKiCg85nbvmN6wHDfV4sXcaHDfxn9w5hT" },
    { "label": "SOL-Reserve-002", "address": "bc1q6n53469l6evqs05apy49pm7r0df0rxhv8cxndj" },
    { "label": "SOL-Reserve-003", "address": "bc1qng4nps9ye7e5pfnd9hy687fygjgr3pq5xhsu4a" },
    { "label": "SOL-Reserve-004", "address": "bc1qapn7vxns6g52xcleljfauwmfy46atz462drfenkvafs5flwmmaasgeavdw" },
    { "label": "SOL-Reserve-005", "address": "bc1qvmvyka72lmwmh7m7465m4rvrytnjgkz03ry8n6" },
    { "label": "SOL-Reserve-006", "address": "bc1q5067xvf70n668vsh7p5rxglpjws7jahw2vr8hu" },
    { "label": "SOL-Reserve-007", "address": "bc1qr6ca6ve657wxx069z5g9gaw52gftd7sse3245a" }
  ],
  "attestation": {
    "method": "ed25519-offchain-signature",
    "pubkey_base58": "FILL_WITH_OCOS_SOL_AUTHORITY_PUBKEY",
    "signature_base64": "FILL_AFTER_SIGN"
  }
}
JSON

# 3) İmzalama skripti (ed25519, tweetnacl)
cat > tools/solana/sign_reserves.ts <<'TS'
import { readFileSync, writeFileSync } from "fs";
import nacl from "tweetnacl";

/**
 * Sign Solana reserve JSON with an ed25519 keypair.
 * SECRET_KEY: base64 (64-byte ed25519 secret key)
 *
 * Usage:
 * SECRET_KEY=<base64> ts-node tools/solana/sign_reserves.ts non_evm/solana/RESERVE_ADDRESSES_SOLANA.json
 */
function b64ToU8(b64: string): Uint8Array { return Buffer.from(b64, "base64"); }
function u8ToB64(u8: Uint8Array): string { return Buffer.from(u8).toString("base64"); }

const file = process.argv[2] || "non_evm/solana/RESERVE_ADDRESSES_SOLANA.json";
const secretB64 = process.env.SECRET_KEY;
if (!secretB64) throw new Error("Set SECRET_KEY=<base64 64-byte secret key>");

const secretKey = b64ToU8(secretB64);
if (secretKey.length !== 64) throw new Error("SECRET_KEY must be 64 bytes (ed25519)");

const kp = nacl.sign.keyPair.fromSecretKey(secretKey);

const doc = JSON.parse(readFileSync(file, "utf8"));
const payload = JSON.stringify({
  network: doc.network,
  as_of: doc.as_of,
  chain: doc.chain,
  addresses: doc.addresses,
}, null, 2);

const sig = nacl.sign.detached(Buffer.from(payload, "utf8"), kp.secretKey);
doc.attestation = {
  method: "ed25519-offchain-signature",
  pubkey_base58: doc.attestation?.pubkey_base58 || "FILL_WITH_OCOS_SOL_AUTHORITY_PUBKEY",
  signature_base64: u8ToB64(sig),
};

writeFileSync(file, JSON.stringify(doc, null, 2));
console.log("Signed:", file);
TS

# 4) Verifikasiya skripti
cat > tools/solana/verify_reserves.ts <<'TS'
import { readFileSync } from "fs";
import nacl from "tweetnacl";
import bs58 from "bs58";

/**
 * Verify signature of RESERVE_ADDRESSES_SOLANA.json
 *
 * Usage:
 * ts-node tools/solana/verify_reserves.ts non_evm/solana/RESERVE_ADDRESSES_SOLANA.json
 */
function b64ToU8(b64: string): Uint8Array { return Buffer.from(b64, "base64"); }

const file = process.argv[2] || "non_evm/solana/RESERVE_ADDRESSES_SOLANA.json";
const doc = JSON.parse(readFileSync(file, "utf8"));

const payload = JSON.stringify({
  network: doc.network,
  as_of: doc.as_of,
  chain: doc.chain,
  addresses: doc.addresses,
}, null, 2);

const pub58 = doc.attestation?.pubkey_base58;
const sigB64 = doc.attestation?.signature_base64;
if (!pub58 || !sigB64) throw new Error("Missing pubkey/signature");
const pub = bs58.decode(pub58);
const sig = b64ToU8(sigB64);

const ok = nacl.sign.detached.verify(Buffer.from(payload, "utf8"), sig, pub);
console.log("Signature valid?", ok);
process.exit(ok ? 0 : 1);
TS

# 5) Dashboard üçün patchlər (istəyə görə)
cat > dashboard_patch/index_card.html <<'HTML'
<!-- Paste this card inside your dashboard <section class="cards"> -->
<div class="card">
  <h2>Solana & Non-EVM Reserve Addresses</h2>
  <table id="addr-sol">
    <thead><tr><th>Label</th><th>Address</th></tr></thead>
    <tbody></tbody>
  </table>
</div>
HTML

cat > dashboard_patch/app_append.js <<'JS'
// Append this to your dashboard/app.js
async function loadSolanaAddresses() {
  try {
    const res = await fetch('../non_evm/solana/RESERVE_ADDRESSES_SOLANA.json');
    const data = await res.json();
    const tbody = document.querySelector('#addr-sol tbody');
    if (!tbody) return;
    tbody.innerHTML = '';
    (data.addresses || []).forEach(a => {
      const tr = document.createElement('tr');
      tr.innerHTML = `<td>${a.label || ''}</td><td><code>${a.address}</code></td>`;
      tbody.appendChild(tr);
    });
  } catch (e) {
    console.error('Failed to load Solana reserves', e);
  }
}
(async function initSolana(){ await loadSolanaAddresses(); })();
JS

echo "Done. Next steps:"
echo "1) npm i -S tweetnacl bs58"
echo "2) SECRET_KEY=<base64_64byte_ed25519> ts-node tools/solana/sign_reserves.ts non_evm/solana/RESERVE_ADDRESSES_SOLANA.json"
echo "3) ts-node tools/solana/verify_reserves.ts non_evm/solana/RESERVE_ADDRESSES_SOLANA.json"
echo "4) (Optional) Add the dashboard_patch snippets into your dashboard."
