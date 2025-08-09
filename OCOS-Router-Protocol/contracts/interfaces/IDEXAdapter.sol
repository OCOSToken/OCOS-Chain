// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/**
 * @title IDEXAdapter
 * @notice Unified adapter interface for routing swaps across heterogeneous DEXs
 *         (e.g., Uniswap V2/V3, Pancake, TraderJoe, Camelot, etc.).
 *
 * @dev
 * ## Design goals
 * - Stable ABI across different DEX backends
 * - Covers both "exact-in" and "exact-out" flows
 * - Explicit quoting vs execution separation
 * - No assumptions about fee-on-transfer (FOT): implementations MUST handle it
 * - Works as a pure execution primitive for higher-level routers
 *
 * ## Token approvals
 * - Callers (Routers) MUST approve this adapter to spend `tokenIn` *before* swap*,
 *   unless a permit-style method is used (`swapExactInWithPermit`, etc.).
 *
 * ## Slippage & deadlines
 * - Adapter MUST enforce `minOut` for exact-in and `maxIn` for exact-out.
 * - Optional deadline is carried inside `dexData` (see ABI below).
 *
 * ## dexData ABI (recommended schema)
 * Implementations SHOULD accept the following canonical encoding, but may also
 * accept their own variant if documented. Routers are expected to compose the
 * correct payload per DEX.
 *
 *  type DexDataV1 = tuple(
 *      uint8   dexType,        // 0: UniV2-like, 1: UniV3-like, 2: Aggregator, ...
 *      address router,         // target DEX router/quoter/pool or aggregator
 *      bytes   path,           // path/pools encoding (varies by dexType)
 *      uint256 deadline,       // 0 means "no deadline check"
 *      bytes   extra           // per-DEX custom params (e.g., fee tiers, flags)
 *  )
 *
 * Examples:
 * - UniV2-like:
 *     path = abi.encode(address[](tokens))  OR  abi.encodePacked(address0, address1, ...)
 * - UniV3-like:
 *     path = bytes encoded via Uniswap V3 path format (token,fee,token,fee,...,token)
 * - Aggregators:
 *     path/extra are aggregator-specific routes/commands.
 *
 * ## Return value semantics
 * - quote*(): MUST return a best-effort estimate without state mutation.
 * - swap*(): MUST return the final settled amount after transfers.
 *
 * ## Events
 * - Implementations SHOULD emit SwapExecuted to aid off-chain accounting.
 */
interface IDEXAdapter {
    /*//////////////////////////////////////////////////////////////
                               ERRORS
    //////////////////////////////////////////////////////////////*/

    /// @notice Thrown when requested slippage protection is violated.
    error Slippage();

    /// @notice Thrown when a provided deadline has passed.
    error Deadline();

    /// @notice Thrown when provided path/pool encoding is invalid for the target DEX.
    error InvalidPath();

    /// @notice Thrown when the adapter cannot route the pair on the designated DEX.
    error UnsupportedPair();

    /// @notice Thrown when the underlying DEX call bubbles up an error.
    error RouterError(bytes lowLevelData);

    /*//////////////////////////////////////////////////////////////
                                EVENTS
    //////////////////////////////////////////////////////////////*/

    /**
     * @dev Emitted after a successful swap execution.
     * @param user        The initiator of the swap (typically RouterCore).
     * @param tokenIn     ERC20 sold.
     * @param tokenOut    ERC20 bought.
     * @param amountIn    Amount of tokenIn spent (actual).
     * @param amountOut   Amount of tokenOut received (actual).
     * @param router      DEX router/pool address used.
     * @param dexType     DEX type discriminator (implementation-defined).
     * @param dataHash    keccak256 of the `dexData` payload that parameterized the swap.
     */
    event SwapExecuted(
        address indexed user,
        address indexed tokenIn,
        address indexed tokenOut,
        uint256 amountIn,
        uint256 amountOut,
        address router,
        uint8   dexType,
        bytes32 dataHash
    );

    /*//////////////////////////////////////////////////////////////
                           METADATA / DISCOVERY
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Human-readable adapter name (e.g., "UniswapV2Adapter", "PancakeV3Adapter").
     */
    function name() external view returns (string memory);

    /**
     * @notice Semantic version string (e.g., "1.0.0").
     */
    function version() external view returns (string memory);

    /**
     * @notice Implementation-defined DEX family/type discriminator.
     * @dev Suggested mapping:
     *  0: UniV2-like, 1: UniV3-like, 2: Aggregator, 3+: vendor-specific.
     */
    function dexType() external view returns (uint8);

    /**
     * @notice Primary DEX router/quoter/pool address (if applicable).
     * @dev For adapters that multiplex multiple routers, MAY return address(0).
     */
    function defaultRouter() external view returns (address);

    /*//////////////////////////////////////////////////////////////
                                QUOTING
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Get an estimated output for an "exact-in" swap.
     * @param tokenIn   ERC20 to sell.
     * @param tokenOut  ERC20 to buy.
     * @param amountIn  Input amount.
     * @param dexData   ABI-encoded DexDataV1 or implementation-specific payload.
     * @return amountOut Estimated output (not guaranteed).
     */
    function quoteExactIn(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        bytes calldata dexData
    ) external view returns (uint256 amountOut);

    /**
     * @notice Get an estimated required input for an "exact-out" swap.
     * @param tokenIn    ERC20 to sell.
     * @param tokenOut   ERC20 to buy.
     * @param amountOut  Desired output amount.
     * @param dexData    ABI-encoded DexDataV1 or implementation-specific payload.
     * @return amountIn  Estimated input required (not guaranteed).
     */
    function quoteExactOut(
        address tokenIn,
        address tokenOut,
        uint256 amountOut,
        bytes calldata dexData
    ) external view returns (uint256 amountIn);

    /*//////////////////////////////////////////////////////////////
                              EXECUTION
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Execute an "exact-in" swap.
     * @dev
     * - Caller MUST have transferred/approved `amountIn` of `tokenIn` to this adapter
     *   (unless a permit-style function is used).
     * - MUST revert with Slippage() if final out < `minOut`.
     * - SHOULD support fee-on-transfer tokens (spend at most available balance).
     *
     * @param tokenIn   ERC20 to sell.
     * @param tokenOut  ERC20 to buy.
     * @param amountIn  Max input to spend (actual spend may be lower for FOT tokens is undefined; assume equal).
     * @param minOut    Minimum acceptable output (slippage guard).
     * @param dexData   ABI-encoded DexDataV1 or implementation-specific payload.
     * @return amountOut Final output received.
     */
    function swapExactIn(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minOut,
        bytes calldata dexData
    ) external returns (uint256 amountOut);

    /**
     * @notice Execute an "exact-out" swap.
     * @dev
     * - Caller MUST have approved sufficient `tokenIn`.
     * - MUST revert with Slippage() if final in > `maxIn`.
     * - SHOULD support fee-on-transfer tokens by capping total spend to `maxIn`.
     *
     * @param tokenIn    ERC20 to sell.
     * @param tokenOut   ERC20 to buy.
     * @param amountOut  Target output amount (exact).
     * @param maxIn      Max acceptable input spend (slippage guard).
     * @param dexData    ABI-encoded DexDataV1 or implementation-specific payload.
     * @return amountIn  Final input spent.
     */
    function swapExactOut(
        address tokenIn,
        address tokenOut,
        uint256 amountOut,
        uint256 maxIn,
        bytes calldata dexData
    ) external returns (uint256 amountIn);

    /*//////////////////////////////////////////////////////////////
                        OPTIONAL: PERMIT-ASSISTED FLOWS
       (Implementations MAY revert if not supported; routers can feature-detect)
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Exact-in swap with EIP-2612 permit in the same tx.
     * @dev The adapter should call permit() on tokenIn before pulling funds.
     * @param permitData ABI-encoded arguments for tokenIn.permit(...)
     */
    function swapExactInWithPermit(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minOut,
        bytes calldata dexData,
        bytes calldata permitData
    ) external returns (uint256 amountOut);

    /**
     * @notice Exact-in swap using Uniswap Permit2 (if supported by router/caller).
     * @dev The adapter should validate Permit2 signature and pull funds via Permit2.
     * @param permit2Data ABI-encoded payload for Permit2 permit+transfer.
     */
    function swapExactInWithPermit2(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minOut,
        bytes calldata dexData,
        bytes calldata permit2Data
    ) external returns (uint256 amountOut);

    /*//////////////////////////////////////////////////////////////
                           FEATURE DISCOVERY (OPTIONAL)
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Lightweight feature detection for optional methods.
     * @dev featureId examples:
     *  - bytes4(keccak256("swapExactInWithPermit(address,address,uint256,uint256,bytes,bytes)"))
     *  - bytes4(keccak256("swapExactInWithPermit2(address,address,uint256,uint256,bytes,bytes)"))
     */
    function supportsFeature(bytes4 featureId) external view returns (bool);
}
