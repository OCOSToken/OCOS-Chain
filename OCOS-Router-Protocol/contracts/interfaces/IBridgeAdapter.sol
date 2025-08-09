// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/**
 * @title IBridgeAdapter
 * @notice Standard interface for pluggable cross-chain bridge adapters used by OCOS Router.
 *
 * @dev Design goals:
 * - Stable, audit-friendly ABI with clear semantics for quoting, sending, and delivering messages.
 * - No external imports to avoid "file import callback" problems in single-file deployments/Remix.
 * - Explicit message struct (ORPMessage) shared with Router; adapters MUST NOT change fields.
 * - Deterministic message identifiers (msgId) that the adapter implementation returns on send()
 *   and reuses on deliver() to prevent replay and double-consumption.
 *
 * SECURITY MODEL (high-level):
 * - Router trusts only adapters on its allowlist to call back deliver-like entrypoints on destination.
 * - Adapters are responsible for interacting with their underlying bridge vendor (CCIP/LayerZero/Axelar/...),
 *   paying vendor fees (via value forwarded by Router), and receiving vendor callbacks on the destination chain.
 * - Adapters MUST ensure one-time delivery guarantees and emit canonical events to support indexing/forensics.
 *
 * TERMINOLOGY:
 * - "Source chain": the chain on which send() is called; tokens are locked/burned here.
 * - "Destination chain": the chain on which the vendor callback occurs; tokens are unlocked/minted here,
 *    then the adapter calls Router.onORPMessage(...) (or equivalent) to continue the route.
 */
interface IBridgeAdapter {
    // =========
    // Structs
    // =========

    /**
     * @notice Canonical ORP cross-chain message envelope.
     * @dev EXACTLY matches Router's definition. Keep fields and ordering identical.
     *      Encoded once and passed to vendor bridge; msgId is computed over this envelope (and optionally vendor domain).
     */
    struct ORPMessage {
        uint256 version;        // Protocol version (1)
        uint256 srcChainId;     // EVM chain id where send() is invoked
        uint256 dstChainId;     // EVM chain id for destination (or vendor's mapped ID)
        bytes32 routeId;        // keccak256 of immutable route config
        address srcSender;      // User on source chain (Router forwards msg.sender)
        bytes   receiver;       // abi.encode(dst EVM address) or vendor-specific addressing bytes
        bytes   token;          // abi.encode(EVM token address on dst) or vendor-specific asset tag
        uint256 amountIn;       // Amount to bridge (post-fee, if Router fees are collected pre-bridge)
        uint256 minAmountOut;   // Slippage guard for dst execution
        uint256 deadline;       // Unix time: message is invalid after this timestamp
        uint256 nonce;          // Per-user/route anti-replay nonce
        bytes   callData;       // Optional: dst program call (e.g., deposit/stake)
        bytes   aux;            // Optional: extra metadata (e.g., tokenOut on dst)
    }

    /**
     * @notice Parameters for fee quoting and sending.
     * @dev Extendable bag to avoid breaking changes in the interface.
     */
    struct BridgeSendParams {
        // Receiver gas on destination for vendor execution (if applicable).
        // Some vendors require specifying gasLimit for dst hook.
        uint256 dstGasLimit;

        // Refund address for any unused native value returned by the vendor (if supported).
        address refundAddress;

        // Optional vendor-specific configuration (serialization format free-form).
        // Examples: lane/channel ID, ZK proof options, strict sequencing flags, AA sponsor flags, etc.
        bytes vendorConfig;
    }

    /**
     * @notice Normalized fee quote returned by adapters.
     * @dev Vendors price in various units; normalize to native fee + optional token fee for transparency.
     */
    struct BridgeFeeQuote {
        // Native currency (e.g., ETH/BNB/MATIC) expected to be provided in msg.value for send().
        uint256 nativeFee;

        // Optional fee taken in tokens (rare; present for certain vendors). 0 if not used.
        uint256 tokenFee;

        // Opaque blob echoing vendor internals (e.g., lane quotes) for off-chain debugging.
        bytes quoteData;
    }

    /**
     * @notice Delivery metadata observed on destination chain during vendor callback.
     * @dev Adapters SHOULD populate and emit this via events for forensic traceability.
     */
    struct DeliveryContext {
        bytes32 msgId;          // Adapter-computed deterministic message id
        uint256 srcChainId;     // Source EVM id
        uint256 dstChainId;     // Destination EVM id (current chain)
        address executor;       // msg.sender of the vendor → adapter entrypoint (vendor router/endpoint)
        bytes   vendorReceipt;  // Opaque vendor receipt/proof id if available
    }

    // =======
    // Events
    // =======

    /// @notice Emitted when a message is accepted for bridging on the source chain.
    event BridgeMessageSent(
        bytes32 indexed msgId,
        uint256 indexed srcChainId,
        uint256 indexed dstChainId,
        address srcSender,
        uint256 amountIn,
        uint256 nativeFeeQuoted,
        bytes   vendorData
    );

    /// @notice Emitted on destination chain upon successful delivery before invoking the Router.
    event BridgeMessageDelivered(
        bytes32 indexed msgId,
        uint256 indexed srcChainId,
        uint256 indexed dstChainId,
        address adapterExecutor,
        bytes   vendorReceipt
    );

    /// @notice Emitted on destination chain if vendor callback is received but Router continuation fails.
    /// @dev The adapter MUST ensure funds safety before emitting failure (e.g., keep funds in adapter/Vault).
    event BridgeMessageFailed(
        bytes32 indexed msgId,
        uint256 indexed srcChainId,
        uint256 indexed dstChainId,
        address adapterExecutor,
        bytes   reason // ABI-encoded revert or error bytes
    );

    // ======
    // Errors
    // ======

    error InvalidDeadline();
    error InvalidChainDomain();
    error InsufficientFee();
    error UnauthorizedExecutor(address executor);
    error AlreadyProcessed(bytes32 msgId);
    error VendorError(bytes reason);
    error RouterNotSet();
    error ZeroAddress();

    // ==========================
    // Immutable / configuration
    // ==========================

    /**
     * @notice Bridge provider identifier, keccak256("CCIP"), keccak256("LAYERZERO"), etc.
     * @dev Prefer a constant value per implementation for off-chain discovery.
     */
    function providerId() external view returns (bytes32);

    /**
     * @notice Human-readable adapter name, e.g., "CCIPAdapter v1".
     */
    function name() external view returns (string memory);

    /**
     * @notice Address of the Router (destination-side) this adapter will call.
     * @dev Implementations MAY be deployed per-router for stricter isolation. If not set, send/deliver MUST revert.
     */
    function router() external view returns (address);

    /**
     * @notice Returns true if the given address is an authorized vendor executor (endpoint/router) on this chain.
     * @dev Used by Router operators to verify adapter’s protection against spoofed callbacks.
     */
    function isTrustedVendorExecutor(address executor) external view returns (bool);

    // ======================
    // Fee quote & dispatch
    // ======================

    /**
     * @notice Returns a vendor-backed fee quote for sending `m` with given `params`.
     * @dev MUST NOT change state. Use static calls into vendor or deterministic estimation.
     *
     * @param m       Canonical ORP message envelope.
     * @param params  Bridge send params: gas limit, refund address, vendor config.
     *
     * @return q      Normalized fee quote (native/token/opaque).
     */
    function quoteSend(ORPMessage calldata m, BridgeSendParams calldata params)
        external
        view
        returns (BridgeFeeQuote memory q);

    /**
     * @notice Dispatches `m` to destination chain via the underlying vendor.
     * @dev msg.value MUST be >= q.nativeFee from quoteSend, unless the vendor allows zero-fee paths.
     *      MUST compute and return a deterministic msgId derived from `m` and (optionally) vendor domain.
     *      MUST emit BridgeMessageSent.
     *
     * SECURITY:
     * - Implementations MUST validate m.deadline and m.srcChainId == block.chainid.
     * - Implementations MUST ensure funds (m.amountIn) are already transferred/approved by Router to this adapter,
     *   OR document a clear flow if adapter pulls funds itself. In OCOS, Router approves the adapter/bridge.
     *
     * @param m       Canonical ORP message envelope.
     * @param params  Bridge send params: gas limit, refund address, vendor config.
     *
     * @return msgId  Deterministic message id to correlate with delivery on destination.
     */
    function send(ORPMessage calldata m, BridgeSendParams calldata params)
        external
        payable
        returns (bytes32 msgId);

    // ===============================
    // Destination-side vendor hook(s)
    // ===============================

    /**
     * @notice Vendor → Adapter entrypoint on destination chain.
     * @dev Implementations typically expose a public function that the vendor’s router/endpoint calls.
     *      That function SHOULD:
     *        1) authenticate msg.sender as a trusted vendor executor,
     *        2) decode vendor payload → ORPMessage `m`,
     *        3) mint/unlock assets for this adapter or for the Router (as per vendor model),
     *        4) compute the SAME msgId as on source,
     *        5) emit BridgeMessageDelivered,
     *        6) call Router.onORPMessage(msgId, m, dex, dexData) or equivalent continuation.
     *
     * NOTE:
     * - This interface does not fix the exact function signature, because vendors differ.
     *   However, adapters MUST expose at least one public entrypoint for the vendor callback
     *   and MUST implement the events/error model above.
     */

    // =================
    // Introspection
    // =================

    /**
     * @notice Returns whether a given message id has been processed on this chain.
     * @dev Adapters MUST implement idempotency and set this flag after a successful continuation to Router.
     */
    function isProcessed(bytes32 msgId) external view returns (bool);

    /**
     * @notice Deterministically computes the message id an implementation would assign to `m`.
     * @dev Useful for precomputation and off-chain reconciliation. MUST match the value returned by send().
     */
    function computeMsgId(ORPMessage calldata m) external view returns (bytes32);

    // =========================
    // Optional management hooks
    // =========================

    /**
     * @notice (Optional) Admin view: returns vendor-specific configuration blob for diagnostics.
     */
    function vendorConfig() external view returns (bytes memory);

    /**
     * @notice (Optional) Admin view: mapping from EVM chain id → vendor domain id, if applicable.
     * @dev Return 0 if not applicable.
     */
    function vendorDomainOf(uint256 evmChainId) external view returns (uint256);
}
