import { readFileSync } from "fs";
import { ethers } from "hardhat";

async function main() {
  const [caller] = await ethers.getSigners();
  const REGISTRY = process.argv[2]; // BTCReserveRegistry address
  if (!REGISTRY) throw new Error("Usage: npx hardhat run scripts/register_reserves.ts --network <net> <RegistryAddress>");

  const abi = [
    "function add(string addr) external",
    "function listedByString(string addr) external view returns (bool)"
  ];
  const reg = new ethers.Contract(REGISTRY, abi, caller);

  const json = JSON.parse(readFileSync("docs/RESERVE_ADDRESSES.json", "utf8"));
  const addrs: string[] = (json.addresses || []).map((a: any) => a.address);
  // dedupe just in case
  const uniq = Array.from(new Set(addrs));

  for (const a of uniq) {
    const already = await reg.listedByString(a);
    if (already) {
      console.log("skip (already):", a);
      continue;
    }
    const tx = await reg.add(a);
    console.log("add:", a, tx.hash);
    await tx.wait();
  }
  console.log("done.");
}

main().catch((e) => { console.error(e); process.exit(1); });
