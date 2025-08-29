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
