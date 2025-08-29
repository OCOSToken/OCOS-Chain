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
