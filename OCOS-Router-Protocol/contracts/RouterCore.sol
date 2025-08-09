// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/**
 * @title OCOS Router Protocol - RouterCore
 * @notice Modular router that supports:
 *         - Intra-chain routing via pluggable DEX adapters
 *         - Cross-chain routing via pluggable bridge adapters
 *
 * Security highlights:
 * - Adapter allowlist (trusted callers) on destination chain entrypoint
 * - Replay protection: msgId consumed map
 * - Deadline & chain-domain checks
 * - Optional protocol fee with bounds
 * - Simple per-token rate limiting window
 * - Pausable by guardian; minimal ReentrancyGuard
 *
 * NOTE: This file is self-contained (no external imports) to simplify bootstrapping.
 *       For production, you may swap the internal libs with audited equivalents.
 */

/*//////////////////////////////////////////////////////////////
                           INTERFACES
//////////////////////////////////////////////////////////////*/

interface IERC20 {
    function decimals() external view returns (uint8);
    function approve(address spender, uint256 value) external returns (bool);
    function transfer(address to, uint256 value) external returns (bool);
    function transferFrom(address from, address to, uint256 value) external returns (bool);
    function balanceOf(address account) external view returns (uint256);
}

interface IDEXAdapter {
    /// @dev Implement adapter for a concrete DEX (e.g., UniswapV2/V3, Pancake)
    function swapExactIn(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minOut,
        bytes calldata dexData
    ) external returns (uint256 amountOut);
}

interface IBridgeAdapter {
    /// @notice Quote bridge fee payable in native token on the source chain
    function quoteSend(bytes calldata orpMessage) external view returns (uint256 fee);

    /// @notice Bridge message + token (lock/mint, etc.) to destination chain
    /// @dev Adapter should transfer/mint assets to RouterCore on destination and then call onORPMessage
    function send(bytes calldata orpMessage) external payable returns (bytes32 msgId);
}

/*//////////////////////////////////////////////////////////////
                           MESSAGE TYPE
//////////////////////////////////////////////////////////////*/

/// @notice Cross-chain message descriptor (chain-agnostic payload)
struct ORPMessage {
    uint256 version;       // 1
    uint256 srcChainId;    // must equal block.chainid on source
    uint256 dstChainId;
    bytes32 routeId;       // keccak(config)
    address srcSender;     // msg.sender on source
    bytes   receiver;      // EVM: abi.encode(address)
    bytes   token;         // EVM: abi.encode(address tokenIn on dst or wrapped)
    uint256 amountIn;      // amount delivered to dst (before DEX)
    uint256 minAmountOut;  // slippage guard for dst swap
    uint256 deadline;      // unix time
    uint256 nonce;         // user-controlled or bridge-provided
    bytes   callData;      // optional: call-after-swap on dst
    bytes   aux;           // EVM: abi.encode(address tokenOut on dst) or extra meta
}

/*//////////////////////////////////////////////////////////////
                     INTERNAL UTILS / LIBRARIES
//////////////////////////////////////////////////////////////*/

/// @dev Minimal, revert-on-false safe transfers
library SafeTransferLib {
    function _call(address token, bytes memory data) private returns (bool ok) {
        // solhint-disable-next-line avoid-low-level-calls
        (ok, bytes memory ret) = token.call(data);
        return ok && (ret.length == 0 || abi.decode(ret, (bool)));
    }

    function safeApprove(IERC20 t, address to, uint256 amount) internal {
        require(_call(address(t), abi.encodeWithSelector(IERC20.approve.selector, to, amount)), "APPROVE_FAIL");
    }

    function safeTransfer(IERC20 t, address to, uint256 amount) internal {
        require(_call(address(t), abi.encodeWithSelector(IERC20.transfer.selector, to, amount)), "TRANSFER_FAIL");
    }

    function safeTransferFrom(IERC20 t, address from, address to, uint256 amount) internal {
        require(_call(address(t), abi.encodeWithSelector(IERC20.transferFrom.selector, from, to, amount)), "TFROM_FAIL");
    }
}

/// @dev Minimal ReentrancyGuard
abstract contract ReentrancyGuard {
    uint256 private _status; // 0 = uninit, 1 = entered, 2 = not-entered
    constructor() { _status = 2; }
    modifier nonReentrant() {
        require(_status != 1, "REENTRANCY");
        _status = 1;
        _;
        _status = 2;
    }
}

/*//////////////////////////////////////////////////////////////
                          ROUTER CORE
//////////////////////////////////////////////////////////////*/

contract RouterCore is ReentrancyGuard {
    using SafeTransferLib for IERC20;

    /*//////////////////////////////////////////////////////////////
                                ERRORS
    //////////////////////////////////////////////////////////////*/
    error Deadline();
    error InvalidChain();
    error Slippage();
    error AlreadyProcessed();
    error NotAllowed();
    error Paused();
    error FeeTooHigh();
    error ZeroAddress();
    error LimitExceeded();

    /*//////////////////////////////////////////////////////////////
                                EVENTS
    //////////////////////////////////////////////////////////////*/
    event IntraRouted(
        address indexed user,
        address indexed tokenIn,
        address indexed tokenOut,
        uint256 amountIn,
        uint256 amountOut
    );

    event CrossSent(
        bytes32 indexed msgId,
        uint256 dstChainId,
        address indexed user,
        address indexed tokenIn,
        uint256 amountIn,
        uint256 bridgeFee
    );

    event CrossReceived(
        bytes32 indexed msgId,
        uint256 srcChainId,
        address indexed tokenOut,
        uint256 amountOut,
        address receiver
    );

    event AdapterAllowed(address indexed adapter, bool allowed);
    event PausedStatus(bool status);
    event ProtocolFeeUpdated(uint16 bps, address indexed recipient);
    event GuardianUpdated(address indexed oldGuardian, address indexed newGuardian);
    event FeeManagerUpdated(address indexed oldFeeManager, address indexed newFeeManager);
    event TokenLimitUpdated(address indexed token, uint256 window, uint256 maxOut);

    /*//////////////////////////////////////////////////////////////
                           STORAGE / CONFIG
    //////////////////////////////////////////////////////////////*/

    // Admins
    address public feeManager; // entity allowed to update fee params
    address public guardian;   // can pause & manage adapters

    // Pausable
    bool public paused;

    // Adapter allowlist: who can call onORPMessage on destination chain
    mapping(address => bool) public adapterAllowlist;

    // Replay protection: message id consumed
    mapping(bytes32 => bool) public processed;

    // Protocol fee (taken from output before sending to receiver)
    // bps out of 10_000; e.g., 15 = 0.15%
    uint16 public protocolFeeBps;            // bounded by MAX_FEE_BPS
    address public protocolFeeRecipient;     // must be non-zero if protocolFeeBps > 0
    uint16 public constant MAX_FEE_BPS = 50; // 0.50% hard cap

    // Simple per-token rate limiter over a sliding window
    struct TokenLimit {
        uint256 windowSeconds;     // e.g., 86400
        uint256 maxOutInWindow;    // token units
        uint256 windowStart;       // last window start timestamp
        uint256 outInWindow;       // cumulative out since windowStart
    }
    mapping(address => TokenLimit) public tokenLimits;

    /*//////////////////////////////////////////////////////////////
                                MODIFIERS
    //////////////////////////////////////////////////////////////*/

    modifier onlyGuardian() {
        if (msg.sender != guardian) revert NotAllowed();
        _;
    }

    modifier onlyFeeManager() {
        if (msg.sender != feeManager) revert NotAllowed();
        _;
    }

    modifier whenNotPaused() {
        if (paused) revert Paused();
        _;
    }

    /*//////////////////////////////////////////////////////////////
                              CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    constructor(address _feeManager, address _guardian) {
        if (_feeManager == address(0) || _guardian == address(0)) revert ZeroAddress();
        feeManager = _feeManager;
        guardian   = _guardian;
        protocolFeeRecipient = _feeManager; // sane default
    }

    /*//////////////////////////////////////////////////////////////
                           ADMIN / GOVERNANCE
    //////////////////////////////////////////////////////////////*/

    function setAdapter(address adapter, bool allowed) external onlyGuardian {
        if (adapter == address(0)) revert ZeroAddress();
        adapterAllowlist[adapter] = allowed;
        emit AdapterAllowed(adapter, allowed);
    }

    function pause(bool s) external onlyGuardian {
        paused = s;
        emit PausedStatus(s);
    }

    function setGuardian(address newGuardian) external onlyGuardian {
        if (newGuardian == address(0)) revert ZeroAddress();
        emit GuardianUpdated(guardian, newGuardian);
        guardian = newGuardian;
    }

    function setFeeManager(address newFeeManager) external onlyFeeManager {
        if (newFeeManager == address(0)) revert ZeroAddress();
        emit FeeManagerUpdated(feeManager, newFeeManager);
        feeManager = newFeeManager;
    }

    /// @notice Update protocol fee (bps) and recipient (must be non-zero if bps > 0)
    function setProtocolFee(uint16 bps, address recipient) external onlyFeeManager {
        if (bps > MAX_FEE_BPS) revert FeeTooHigh();
        if (bps > 0 && recipient == address(0)) revert ZeroAddress();
        protocolFeeBps = bps;
        protocolFeeRecipient = recipient;
        emit ProtocolFeeUpdated(bps, recipient);
    }

    /// @notice Configure a simple per-token rate limit over `windowSeconds`
    function setTokenLimit(
        address token,
        uint256 windowSeconds,
        uint256 maxOutInWindow
    ) external onlyGuardian {
        TokenLimit storage L = tokenLimits[token];
        L.windowSeconds = windowSeconds;
        L.maxOutInWindow = maxOutInWindow;
        if (L.windowStart == 0) {
            L.windowStart = block.timestamp;
        }
        emit TokenLimitUpdated(token, windowSeconds, maxOutInWindow);
    }

    /*//////////////////////////////////////////////////////////////
                          INTRA-CHAIN ROUTING
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Execute a same-chain swap path on a trusted DEX adapter
     * @param dex DEX adapter address
     * @param tokenIn ERC20 to spend
     * @param tokenOut ERC20 to receive
     * @param amountIn input amount (from user)
     * @param minOut minimum acceptable output (slippage guard)
     * @param dexData adapter-specific path/pool data
     * @param deadline unix timestamp; must be >= block.timestamp
     */
    function intraRoute(
        address dex,
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minOut,
        bytes calldata dexData,
        uint256 deadline
    )
        external
        whenNotPaused
        nonReentrant
        returns (uint256 out)
    {
        if (block.timestamp > deadline) revert Deadline();

        IERC20 inToken  = IERC20(tokenIn);
        IERC20 outToken = IERC20(tokenOut);

        // Pull funds and approve adapter for exactly amountIn
        inToken.safeTransferFrom(msg.sender, address(this), amountIn);
        inToken.safeApprove(dex, 0); // anti-allowance race
        inToken.safeApprove(dex, amountIn);

        // Execute swap
        out = IDEXAdapter(dex).swapExactIn(tokenIn, tokenOut, amountIn, minOut, dexData);
        if (out < minOut) revert Slippage();

        // Optional protocol fee on output
        out = _takeProtocolFee(out, outToken);

        // Send to the user
        outToken.safeTransfer(msg.sender, out);

        emit IntraRouted(msg.sender, tokenIn, tokenOut, amountIn, out);
    }

    /*//////////////////////////////////////////////////////////////
                          CROSS-CHAIN (SOURCE)
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Source-chain entry to bridge funds + message to destination
     * @dev Adapter is expected to handle asset locking/minting and later call onORPMessage on destination
     * @param bridge bridge adapter address
     * @param m ORPMessage (srcChainId must equal block.chainid)
     */
    function crossRoute(
        address bridge,
        ORPMessage calldata m
    )
        external
        payable
        whenNotPaused
        nonReentrant
        returns (bytes32 msgId)
    {
        if (block.timestamp > m.deadline) revert Deadline();
        if (m.srcChainId != block.chainid) revert InvalidChain();

        address tokenInAddr = abi.decode(m.token, (address));
        IERC20 tokenIn = IERC20(tokenInAddr);

        // Pull funds to router, approve bridge for exactly amountIn
        tokenIn.safeTransferFrom(msg.sender, address(this), m.amountIn);
        tokenIn.safeApprove(bridge, 0);
        tokenIn.safeApprove(bridge, m.amountIn);

        // Encode and quote fee
        bytes memory enc = abi.encode(m);
        uint256 fee = IBridgeAdapter(bridge).quoteSend(enc);

        // Execute bridge call
        msgId = IBridgeAdapter(bridge).send{value: fee}(enc);

        emit CrossSent(msgId, m.dstChainId, msg.sender, tokenInAddr, m.amountIn, fee);
    }

    /*//////////////////////////////////////////////////////////////
                        CROSS-CHAIN (DESTINATION)
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Destination-chain entry called by a trusted adapter after assets are delivered to this contract.
     * @param msgId unique message id (adapter-specific)
     * @param m original ORPMessage
     * @param dex DEX adapter to execute swap on destination
     * @param dexData DEX adapter path/pool data
     *
     * Flow:
     * 1) Adapter must transfer/mint `m.token` of `m.amountIn` to this contract before calling.
     * 2) Router swaps tokenIn -> tokenOut (from m.aux) on the chosen DEX adapter.
     * 3) Applies rate-limit & protocol fee, sends to m.receiver.
     */
    function onORPMessage(
        bytes32 msgId,
        ORPMessage calldata m,
        address dex,
        bytes calldata dexData
    )
        external
        whenNotPaused
        nonReentrant
    {
        if (!adapterAllowlist[msg.sender]) revert NotAllowed();
        if (processed[msgId]) revert AlreadyProcessed();
        processed[msgId] = true;

        address tokenInAddr  = abi.decode(m.token, (address));
        address tokenOutAddr = abi.decode(m.aux, (address));
        IERC20 tokenIn  = IERC20(tokenInAddr);
        IERC20 tokenOut = IERC20(tokenOutAddr);

        // Approve adapter with max (gas-optimized for multiple swaps), safe pattern would zero-reset if needed
        tokenIn.safeApprove(dex, 0);
        tokenIn.safeApprove(dex, type(uint256).max);

        // Execute swap on destination
        uint256 out = IDEXAdapter(dex).swapExactIn(tokenInAddr, tokenOutAddr, m.amountIn, m.minAmountOut, dexData);
        if (out < m.minAmountOut) revert Slippage();

        // Rate limiting on tokenOut
        _enforceTokenLimit(tokenOutAddr, out);

        // Protocol fee
        out = _takeProtocolFee(out, tokenOut);

        // Deliver to receiver
        address receiver = abi.decode(m.receiver, (address));
        tokenOut.safeTransfer(receiver, out);

        emit CrossReceived(msgId, m.srcChainId, tokenOutAddr, out, receiver);

        // OPTIONAL: call-after-swap on receiver (executed by receiver contract pull pattern is safer)
        // If you want push pattern, add a controlled external call here guarded by allowlists.
    }

    /*//////////////////////////////////////////////////////////////
                       INTERNAL: FEE & RATE LIMIT
    //////////////////////////////////////////////////////////////*/

    function _takeProtocolFee(uint256 amountOut, IERC20 tokenOut) internal returns (uint256 net) {
        uint16 bps = protocolFeeBps;
        if (bps == 0) return amountOut;
        uint256 fee = (amountOut * bps) / 10_000;
        if (fee > 0) {
            tokenOut.safeTransfer(protocolFeeRecipient, fee);
            unchecked { net = amountOut - fee; }
        } else {
            net = amountOut;
        }
    }

    function _enforceTokenLimit(address token, uint256 addOut) internal {
        TokenLimit storage L = tokenLimits[token];
        if (L.maxOutInWindow == 0 || L.windowSeconds == 0) return; // disabled

        // roll window
        if (block.timestamp >= L.windowStart + L.windowSeconds) {
            // Start a fresh window aligned to now (or step windows if multiple elapsed)
            uint256 elapsed = block.timestamp - L.windowStart;
            uint256 steps = (elapsed / L.windowSeconds);
            L.windowStart += (steps * L.windowSeconds);
            L.outInWindow = 0;
        }

        // check & update
        uint256 newOut = L.outInWindow + addOut;
        if (newOut > L.maxOutInWindow) revert LimitExceeded();
        L.outInWindow = newOut;
    }

    /*//////////////////////////////////////////////////////////////
                              VIEW HELPERS
    //////////////////////////////////////////////////////////////*/

    /// @notice Returns whether an adapter is trusted to call onORPMessage
    function isAdapterAllowed(address adapter) external view returns (bool) {
        return adapterAllowlist[adapter];
    }

    /// @notice Returns current token limit state
    function getTokenLimit(address token) external view returns (
        uint256 windowSeconds,
        uint256 maxOutInWindow,
        uint256 windowStart,
        uint256 outInWindow
    ) {
        TokenLimit storage L = tokenLimits[token];
        return (L.windowSeconds, L.maxOutInWindow, L.windowStart, L.outInWindow);
    }
}
