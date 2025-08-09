/* ============================================================================
 * OCOS Router Protocol (ORP) – TypeScript SDK
 * File: packages/sdk/src/index.ts
 * Dependencies: viem@^2
 * Status: Public preview (unaudited). Use on testnets first.
 * ==========================================================================*/

import {
  Address,
  Hex,
  encodeAbiParameters,
  decodeAbiParameters,
  keccak256,
  toHex,
  size,
  isHex,
  concatHex,
  getAddress,
  stringToHex,
} from "viem";

/* ----------------------------------------------------------------------------
 * Types
 * --------------------------------------------------------------------------*/

/** Canonical ORP wire message (mirrors Solidity `ORPMessage`). */
export type ORPMessage = {
  /** message version; keep `1` until breaking change */
  version: bigint;
  /** source EVM chainId (must equal `block.chainid` on source) */
  srcChainId: bigint;
  /** destination chain id (EVM or logical id used by adapter) */
  dstChainId: bigint;
  /** deterministic route discriminator (e.g., keccak(config)) */
  routeId: Hex; // 0x-prefixed 32 bytes
  /** sender on source (EVM address). Non-EVM should be mapped by adapter. */
  srcSender: Address;
  /** destination receiver; EVM: abi.encode(address), Non-EVM: bytes */
  receiver: Hex;
  /** token reference on *destination* hop (post-bridge); EVM: abi.encode(address) */
  token: Hex;
  /** amount intended to move/swap on destination */
  amountIn: bigint;
  /** minimum acceptable output to protect against slippage */
  minAmountOut: bigint;
  /** unix timestamp after which the op must fail (replay bound) */
  deadline: bigint;
  /** per-user/per-route nonce to prevent replay across domains */
  nonce: bigint;
  /** optional: encoded call to execute on destination after swap */
  callData: Hex;
  /** optional: aux field; in EVM swap flow it usually encodes tokenOut address */
  aux: Hex;
};

/** Encoded blob + msgId (transport layer payload). */
export type EncodedMessage = {
  encoded: Hex;
  msgId: Hex; // keccak256(encoded)
};

/** Minimal route configuration to compute a stable routeId. */
export type RouteConfig = {
  srcChainId: bigint;
  dstChainId: bigint;
  srcRouter: Address;
  dstRouter: Address;
  bridgeVendorId?: Hex; // optional: include bridge vendor salt
  pathHint?: Hex; // optional: DEX path fingerprint
};

/** Friendly builder input with numbers where convenient. */
export type BuildInput = Omit<ORPMessage, "version" | "routeId"> & {
  version?: number | bigint; // default: 1
  routeConfig?: RouteConfig; // compute routeId if not provided
  routeId?: Hex; // override computed routeId
};

/* ----------------------------------------------------------------------------
 * Constants & ABI Shapes
 * --------------------------------------------------------------------------*/

const ORP_ABI_SHAPE = [
  { type: "uint256" }, // version
  { type: "uint256" }, // srcChainId
  { type: "uint256" }, // dstChainId
  { type: "bytes32" }, // routeId
  { type: "address" }, // srcSender
  { type: "bytes" },   // receiver
  { type: "bytes" },   // token
  { type: "uint256" }, // amountIn
  { type: "uint256" }, // minAmountOut
  { type: "uint256" }, // deadline
  { type: "uint256" }, // nonce
  { type: "bytes" },   // callData
  { type: "bytes" },   // aux
] as const;

/** EIP-712 type description for off-chain signing (if you choose to sign message blobs). */
export const EIP712_ORP_TYPES = {
  ORPMessage: [
    { name: "version", type: "uint256" },
    { name: "srcChainId", type: "uint256" },
    { name: "dstChainId", type: "uint256" },
    { name: "routeId", type: "bytes32" },
    { name: "srcSender", type: "address" },
    { name: "receiver", type: "bytes" },
    { name: "token", type: "bytes" },
    { name: "amountIn", type: "uint256" },
    { name: "minAmountOut", type: "uint256" },
    { name: "deadline", type: "uint256" },
    { name: "nonce", type: "uint256" },
    { name: "callData", type: "bytes" },
    { name: "aux", type: "bytes" },
  ],
} as const;

/* ----------------------------------------------------------------------------
 * Errors
 * --------------------------------------------------------------------------*/

export class ORPValidationError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "ORPValidationError";
  }
}

/* ----------------------------------------------------------------------------
 * Utilities
 * --------------------------------------------------------------------------*/

/** Compute a deterministic `routeId` from route config. */
export function computeRouteId(cfg: RouteConfig): Hex {
  const payload = encodeAbiParameters(
    [
      { type: "uint256" }, // srcChainId
      { type: "uint256" }, // dstChainId
      { type: "address" }, // srcRouter
      { type: "address" }, // dstRouter
      { type: "bytes32" }, // bridgeVendorId or 0x0..
      { type: "bytes32" }, // pathHint or 0x0..
    ],
    [
      cfg.srcChainId,
      cfg.dstChainId,
      cfg.srcRouter,
      cfg.dstRouter,
      toBytes32(cfg.bridgeVendorId ?? "0x"),
      toBytes32(cfg.pathHint ?? "0x"),
    ]
  );
  return keccak256(payload);
}

/** Returns a future unix timestamp: now + seconds (default 1800s). */
export function deadlineIn(seconds = 1800): bigint {
  return BigInt(Math.floor(Date.now() / 1000) + seconds);
}

/** Compute a conservative minOut based on a basis-point slippage tolerance. */
export function minAmountOutFromBps(amountExpected: bigint, maxSlippageBps: number): bigint {
  if (maxSlippageBps < 0 || maxSlippageBps > 10_000) {
    throw new ORPValidationError("maxSlippageBps must be in [0, 10000]");
  }
  return (amountExpected * BigInt(10_000 - maxSlippageBps)) / 10_000n;
}

/** Ensure a hex is exactly 32 bytes (left-padded zero if empty). */
export function toBytes32(h: Hex): Hex {
  if (!h || h === "0x") return ("0x" + "00".repeat(32)) as Hex;
  if (!isHex(h)) throw new ORPValidationError("toBytes32: not hex");
  const sz = size(h);
  if (sz === 32) return h;
  if (sz > 32) throw new ORPValidationError("toBytes32: exceeds 32 bytes");
  // left pad
  const pad = "0x" + "00".repeat(32 - sz) as Hex;
  return concatHex([pad, h]);
}

/** Encode an Ethereum address into `bytes` (EVM side for receiver/token fields). */
export function encodeAddressToBytes(addr: Address): Hex {
  // abi.encode(address) = left-padded 32-bytes; viem handles via encodeAbiParameters([address])
  return encodeAbiParameters([{ type: "address" }], [getAddress(addr)]);
}

/** Decode an Ethereum address from `bytes` (when you expect it was encoded as address). */
export function decodeAddressFromBytes(bytes_: Hex): Address {
  const [addr] = decodeAbiParameters([{ type: "address" }], bytes_);
  return getAddress(addr as Address);
}

/* ----------------------------------------------------------------------------
 * Encoding / Decoding / Validation
 * --------------------------------------------------------------------------*/

/** Validates basic structural constraints; throws ORPValidationError on issues. */
export function validateORPMessage(m: ORPMessage): void {
  if (m.version <= 0n) throw new ORPValidationError("version must be > 0");
  if (!m.routeId || size(m.routeId) !== 32) throw new ORPValidationError("routeId must be 32 bytes");
  if (!isHex(m.receiver)) throw new ORPValidationError("receiver must be hex-encoded bytes");
  if (!isHex(m.token)) throw new ORPValidationError("token must be hex-encoded bytes");
  if (m.amountIn <= 0n) throw new ORPValidationError("amountIn must be > 0");
  if (m.minAmountOut < 0n) throw new ORPValidationError("minAmountOut must be >= 0");
  if (m.deadline <= 0n) throw new ORPValidationError("deadline must be > 0");
  if (m.nonce < 0n) throw new ORPValidationError("nonce must be >= 0");
  if (!isHex(m.callData)) throw new ORPValidationError("callData must be hex");
  if (!isHex(m.aux)) throw new ORPValidationError("aux must be hex");
}

/** ABI-encodes an ORPMessage and returns the payload + msgId (keccak256). */
export function encodeORPMessage(m: ORPMessage): EncodedMessage {
  validateORPMessage(m);
  const encoded = encodeAbiParameters(ORP_ABI_SHAPE as any, [
    m.version,
    m.srcChainId,
    m.dstChainId,
    m.routeId,
    m.srcSender,
    m.receiver,
    m.token,
    m.amountIn,
    m.minAmountOut,
    m.deadline,
    m.nonce,
    m.callData,
    m.aux,
  ]);
  return { encoded, msgId: keccak256(encoded) };
}

/** Decodes an ORPMessage from ABI-encoded bytes. */
export function decodeORPMessage(encoded: Hex): ORPMessage {
  const [
    version,
    srcChainId,
    dstChainId,
    routeId,
    srcSender,
    receiver,
    token,
    amountIn,
    minAmountOut,
    deadline,
    nonce,
    callData,
    aux,
  ] = decodeAbiParameters(ORP_ABI_SHAPE as any, encoded);
  const m: ORPMessage = {
    version,
    srcChainId,
    dstChainId,
    routeId,
    srcSender,
    receiver,
    token,
    amountIn,
    minAmountOut,
    deadline,
    nonce,
    callData,
    aux,
  };
  validateORPMessage(m);
  return m;
}

/** High-level builder that accepts friendly input and computes routeId if needed. */
export function buildMsg(input: BuildInput): EncodedMessage & { message: ORPMessage } {
  const version = BigInt(input.version ?? 1);
  const routeId =
    input.routeId ??
    (input.routeConfig ? computeRouteId(input.routeConfig) : toBytes32("0x"));
  const message: ORPMessage = {
    version,
    srcChainId: input.srcChainId,
    dstChainId: input.dstChainId,
    routeId,
    srcSender: getAddress(input.srcSender),
    receiver: normalizeHex(input.receiver),
    token: normalizeHex(input.token),
    amountIn: input.amountIn,
    minAmountOut: input.minAmountOut,
    deadline: input.deadline,
    nonce: input.nonce,
    callData: normalizeHex(input.callData),
    aux: normalizeHex(input.aux),
  };
  const { encoded, msgId } = encodeORPMessage(message);
  return { encoded, msgId, message };
}

/* ----------------------------------------------------------------------------
 * EIP-712 Domain Helpers (optional)
 * --------------------------------------------------------------------------*/

/**
 * Builds a canonical EIP-712 domain for signing ORP messages on a given chain.
 * You may choose to bind the signature to a specific RouterCore contract.
 */
export function eip712Domain(
  chainId: bigint,
  verifyingContract: Address,
  name = "OCOS Router Protocol",
  version = "1"
) {
  return {
    name,
    version,
    chainId,
    verifyingContract,
  } as const;
}

/* ----------------------------------------------------------------------------
 * Convenience Helpers
 * --------------------------------------------------------------------------*/

/** Packs tokenOut address into `aux` for the common EVM swap pattern. */
export function auxEncodeTokenOut(tokenOut: Address): Hex {
  return encodeAbiParameters([{ type: "address" }], [getAddress(tokenOut)]);
}

/** Packs receiver address into `bytes` for EVM chains. */
export function receiverEncodeEVM(addr: Address): Hex {
  return encodeAddressToBytes(addr);
}

/** Packs token address (on destination) into `bytes` for EVM chains. */
export function tokenEncodeEVM(addr: Address): Hex {
  return encodeAddressToBytes(addr);
}

/* ----------------------------------------------------------------------------
 * Internal
 * --------------------------------------------------------------------------*/

function normalizeHex(h: Hex): Hex {
  if (!h) return "0x";
  if (!isHex(h)) throw new ORPValidationError("Expected hex string (0x...)");
  return h.toLowerCase() as Hex;
}

/* ----------------------------------------------------------------------------
 * Example Usage (kept here for developer ergonomics)
 * Remove or comment out before publishing to strict environments.
 * --------------------------------------------------------------------------*/
/*
import { buildMsg, deadlineIn, receiverEncodeEVM, tokenEncodeEVM, auxEncodeTokenOut } from "@ocos/orp-sdk";

const { message, encoded, msgId } = buildMsg({
  version: 1,
  srcChainId: 56n, // BSC
  dstChainId: 1n,  // Ethereum
  routeConfig: {
    srcChainId: 56n,
    dstChainId: 1n,
    srcRouter: "0x1111111111111111111111111111111111111111",
    dstRouter: "0x2222222222222222222222222222222222222222",
    bridgeVendorId: "0x" + "00".repeat(32) as Hex,
  },
  srcSender: "0x3333333333333333333333333333333333333333",
  receiver: receiverEncodeEVM("0x4444444444444444444444444444444444444444"),
  token: tokenEncodeEVM("0x5555555555555555555555555555555555555555"),
  amountIn: 1_000_000_000_000_000_000n, // 1e18
  minAmountOut: 990_000_000_000_000_000n,
  deadline: deadlineIn(1800),
  nonce: 1n,
  callData: "0x",
  aux: auxEncodeTokenOut("0x6666666666666666666666666666666666666666"),
});
console.log({ msgId, len: encoded.length });
*/

/* End of file */
