// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/**
 * @title MockBridgeAdapter
 * @notice Test məqsədli körpü adapteri:
 *  - Source zəncirdə RouterCore tərəfindən approve olunduqdan sonra tokeni transferFrom ilə çəkir
 *  - Təyinat (dst) RouterCore ünvanına tokeni köçürür
 *  - Dərhal dst RouterCore.onORPMessage(...) çağırır (adapter allowlist-də olmalıdır)
 *
 * Qeyd: Real körpülərdə aktivin lock/mint mexanizmi zəncirlərarası dəyişir; bu mock yalnız eyni/zidd şəbəkədə
 *       ssenarini imitasiya edir və inteqrasiya testləri üçündür.
 */

/// ====== Minimal ERC20 ======
interface IERC20 {
    function approve(address, uint256) external returns (bool);
    function transfer(address, uint256) external returns (bool);
    function transferFrom(address, address, uint256) external returns (bool);
    function balanceOf(address) external view returns (uint256);
}

/// ====== IBridgeAdapter (RouterCore ilə eyni interfeys) ======
interface IBridgeAdapter {
    function quoteSend(bytes calldata orpMessage) external view returns (uint256 fee);
    function send(bytes calldata orpMessage) external payable returns (bytes32 msgId);
}

/// ====== ORPMessage (RouterCore ilə eyni layout) ======
struct ORPMessage {
    uint256 version;
    uint256 srcChainId;
    uint256 dstChainId;
    bytes32 routeId;
    address srcSender;
    bytes   receiver;     // EVM: abi.encode(address)
    bytes   token;        // EVM: abi.encode(address tokenIn on dst or wrapped)
    uint256 amountIn;
    uint256 minAmountOut;
    uint256 deadline;
    uint256 nonce;
    bytes   callData;
    bytes   aux;          // EVM: abi.encode(address tokenOut on dst)
}

/// ====== Destination RouterCore minimal interfeysi ======
interface IRouterCore {
    function onORPMessage(
        bytes32 msgId,
        ORPMessage calldata m,
        address dex,
        bytes calldata dexData
    ) external;
}

/// ====== Safe transfer helpers (revert-on-false) ======
library SafeTransferLib {
    function _call(address token, bytes memory data) private returns (bool ok) {
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

contract MockBridgeAdapter is IBridgeAdapter {
    using SafeTransferLib for IERC20;

    /*//////////////////////////////////////////////////////////////
                                EVENTS
    //////////////////////////////////////////////////////////////*/
    event DestinationSet(uint256 indexed dstChainId, address indexed router, address dex, bytes dexData);
    event FeeUpdated(uint256 feeWei);
    event Sent(bytes32 indexed msgId, uint256 indexed dstChainId, address indexed token, uint256 amount, address dstRouter);
    event Delivered(bytes32 indexed msgId, uint256 indexed dstChainId, address dstRouter);
    event OwnerTransferred(address indexed oldOwner, address indexed newOwner);
    event FundsRecovered(address indexed token, uint256 amount);

    /*//////////////////////////////////////////////////////////////
                                ERRORS
    //////////////////////////////////////////////////////////////*/
    error NotOwner();
    error ZeroAddress();
    error DstNotConfigured();
    error FeeMismatch();

    /*//////////////////////////////////////////////////////////////
                               OWNERSHIP
    //////////////////////////////////////////////////////////////*/
    address public owner;

    modifier onlyOwner() {
        if (msg.sender != owner) revert NotOwner();
        _;
    }

    /*//////////////////////////////////////////////////////////////
                               STORAGE
    //////////////////////////////////////////////////////////////*/

    struct DestConfig {
        address router; // destination RouterCore
        address dex;    // default DEX adapter to use on destination
        bytes   dexData;
    }

    // dstChainId => destination config
    mapping(uint256 => DestConfig) public dest;

    // flat message fee (wei) returned by quoteSend and required in send{value:...}
    uint256 public feeWei;

    constructor(address _owner) {
        if (_owner == address(0)) revert ZeroAddress();
        owner = _owner;
    }

    /*//////////////////////////////////////////////////////////////
                           ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    function setDestination(uint256 dstChainId, address router, address dex, bytes calldata dexData) external onlyOwner {
        if (router == address(0)) revert ZeroAddress();
        dest[dstChainId] = DestConfig({router: router, dex: dex, dexData: dexData});
        emit DestinationSet(dstChainId, router, dex, dexData);
    }

    function setFeeWei(uint256 newFeeWei) external onlyOwner {
        feeWei = newFeeWei;
        emit FeeUpdated(newFeeWei);
    }

    function transferOwnership(address newOwner) external onlyOwner {
        if (newOwner == address(0)) revert ZeroAddress();
        emit OwnerTransferred(owner, newOwner);
        owner = newOwner;
    }

    /// @notice Recover ETH
    function sweepETH(address payable to) external onlyOwner {
        if (to == address(0)) revert ZeroAddress();
        to.transfer(address(this).balance);
    }

    /// @notice Recover ERC20 mistakenly sent to adapter
    function sweepToken(address token, address to, uint256 amount) external onlyOwner {
        if (to == address(0)) revert ZeroAddress();
        IERC20(token).safeTransfer(to, amount);
        emit FundsRecovered(token, amount);
    }

    /*//////////////////////////////////////////////////////////////
                         IBridgeAdapter IMPLEMENTATION
    //////////////////////////////////////////////////////////////*/

    /// @notice Flat quote
    function quoteSend(bytes calldata /*orpMessage*/) external view returns (uint256 fee) {
        return feeWei;
    }

    /// @notice Pull token from RouterCore, deliver to dst RouterCore, then call onORPMessage
    function send(bytes calldata orpMessage) external payable returns (bytes32 msgId) {
        if (msg.value != feeWei) revert FeeMismatch();

        // decode message
        ORPMessage memory m = abi.decode(orpMessage, (ORPMessage));
        DestConfig memory d = dest[m.dstChainId];
        if (d.router == address(0)) revert DstNotConfigured();

        // token & amount
        address tokenIn = abi.decode(m.token, (address));
        IERC20 t = IERC20(tokenIn);

        // 1) source: pull funds from RouterCore (msg.sender is RouterCore with prior approve)
        t.safeTransferFrom(msg.sender, address(this), m.amountIn);

        // 2) "deliver" to destination router (on same chain in tests)
        t.safeTransfer(d.router, m.amountIn);

        // create a deterministic-ish msgId for logging & replay map
        msgId = keccak256(
            abi.encode(
                address(this),
                msg.sender,
                block.chainid,
                m.srcChainId,
                m.dstChainId,
                m.routeId,
                m.nonce,
                block.number
            )
        );

        emit Sent(msgId, m.dstChainId, tokenIn, m.amountIn, d.router);

        // 3) call destination router entrypoint
        IRouterCore(d.router).onORPMessage(msgId, m, d.dex, d.dexData);

        emit Delivered(msgId, m.dstChainId, d.router);

        return msgId;
    }

    /*//////////////////////////////////////////////////////////////
                               RECEIVE ETH
    //////////////////////////////////////////////////////////////*/
    receive() external payable {}
}
