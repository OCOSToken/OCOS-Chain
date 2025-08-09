// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/**
 * @title CCIPAdapter
 * @notice Chainlink CCIP-based bridge adapter for OCOS Router Protocol (ORP).
 *
 * Flow:
 *  - Source chain:
 *      RouterCore.crossRoute() → this adapter → transfers token via transferFrom() →
 *      sends CCIP message (ORPMessage + token) using ccipSend() to destination adapter address.
 *  - Destination chain:
 *      CCIP Router calls this adapter’s ccipReceive() →
 *      adapter transfers the token to the destination RouterCore →
 *      calls RouterCore.onORPMessage(msgId, ORPMessage, dex, dexData).
 *
 * Security:
 *  - ccipReceive is only accepted from the registered CCIP Router (msg.sender check).
 *  - On the destination RouterCore, this adapter must be on the adapter allowlist.
 */

/// ========== Minimal ERC20 ==========
interface IERC20 {
    function approve(address, uint256) external returns (bool);
    function transfer(address, uint256) external returns (bool);
    function transferFrom(address, address, uint256) external returns (bool);
    function balanceOf(address) external view returns (uint256);
}

/// ========== ORPMessage ==========
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

/// ========== RouterCore Interface ==========
interface IRouterCore {
    function onORPMessage(
        bytes32 msgId,
        ORPMessage calldata m,
        address dex,
        bytes calldata dexData
    ) external;
}

/// ========== CCIP Types ==========
library CCIPClient {
    struct EVMTokenAmount {
        address token;
        uint256 amount;
    }

    struct EVM2AnyMessage {
        bytes receiver;
        bytes data;
        EVMTokenAmount[] tokenAmounts;
        address feeToken;
        bytes extraArgs;
    }

    struct Any2EVMMessage {
        bytes32 messageId;
        uint64  sourceChainSelector;
        bytes   sender;
        bytes   data;
        EVMTokenAmount[] destTokenAmounts;
    }
}

interface IRouterClient {
    function getFee(uint64 destChainSelector, CCIPClient.EVM2AnyMessage calldata message) external view returns (uint256);
    function ccipSend(uint64 destChainSelector, CCIPClient.EVM2AnyMessage calldata message) external payable returns (bytes32);
}

interface IAny2EVMMessageReceiver {
    function ccipReceive(CCIPClient.Any2EVMMessage calldata message) external;
}

/// ====== Safe Transfer Helpers ======
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

contract CCIPAdapter is IAny2EVMMessageReceiver {
    using SafeTransferLib for IERC20;

    /*//////////////////////////////////////////////////////////////
                                 EVENTS
    //////////////////////////////////////////////////////////////*/
    event RouterSet(address indexed router);
    event FeeTokenSet(address indexed feeToken);
    event DestinationSet(uint256 indexed dstChainId, uint64 indexed selector, bytes receiver, address dstRouterCore, address dstDex, bytes dexData);
    event Sent(bytes32 indexed msgId, uint64 indexed selector, address indexed token, uint256 amount, bytes receiver);
    event Delivered(bytes32 indexed msgId, uint64 indexed selector, address dstRouterCore, address tokenOut, uint256 amountOut, address receiver);
    event OwnerTransferred(address indexed oldOwner, address indexed newOwner);
    event FundsRecovered(address indexed token, uint256 amount);

    /*//////////////////////////////////////////////////////////////
                                 ERRORS
    //////////////////////////////////////////////////////////////*/
    error NotOwner();
    error ZeroAddress();
    error RouterNotSet();
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
    IRouterClient public ccipRouter;
    address public feeToken; // 0 = native

    struct DestConfig {
        uint64  selector;
        bytes   receiver;
        address routerCore;
        address dex;
        bytes   dexData;
    }
    mapping(uint256 => DestConfig) public dest;

    constructor(address _owner, address _ccipRouter, address _feeToken) {
        if (_owner == address(0)) revert ZeroAddress();
        owner = _owner;
        if (_ccipRouter != address(0)) {
            ccipRouter = IRouterClient(_ccipRouter);
            emit RouterSet(_ccipRouter);
        }
        feeToken = _feeToken;
        emit FeeTokenSet(_feeToken);
    }

    /*//////////////////////////////////////////////////////////////
                             ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/
    function setRouter(address _router) external onlyOwner {
        if (_router == address(0)) revert ZeroAddress();
        ccipRouter = IRouterClient(_router);
        emit RouterSet(_router);
    }

    function setFeeToken(address _feeToken) external onlyOwner {
        feeToken = _feeToken;
        emit FeeTokenSet(_feeToken);
    }

    function setDestination(
        uint256 dstChainId,
        uint64 selector,
        bytes calldata receiver,
        address dstRouterCore,
        address dstDex,
        bytes calldata dexData
    ) external onlyOwner {
        if (receiver.length == 0 || dstRouterCore == address(0)) revert ZeroAddress();
        dest[dstChainId] = DestConfig({
            selector: selector,
            receiver: receiver,
            routerCore: dstRouterCore,
            dex: dstDex,
            dexData: dexData
        });
        emit DestinationSet(dstChainId, selector, receiver, dstRouterCore, dstDex, dexData);
    }

    function transferOwnership(address newOwner) external onlyOwner {
        if (newOwner == address(0)) revert ZeroAddress();
        emit OwnerTransferred(owner, newOwner);
        owner = newOwner;
    }

    function sweepETH(address payable to) external onlyOwner {
        if (to == address(0)) revert ZeroAddress();
        to.transfer(address(this).balance);
    }

    function sweepToken(address token, address to, uint256 amount) external onlyOwner {
        if (to == address(0)) revert ZeroAddress();
        IERC20(token).safeTransfer(to, amount);
        emit FundsRecovered(token, amount);
    }

    /*//////////////////////////////////////////////////////////////
                      IBridgeAdapter-LIKE API
    //////////////////////////////////////////////////////////////*/
    function quoteSend(bytes calldata orpMessage) external view returns (uint256 fee) {
        if (address(ccipRouter) == address(0)) revert RouterNotSet();
        ORPMessage memory m = abi.decode(orpMessage, (ORPMessage));
        DestConfig memory d = dest[m.dstChainId];
        if (d.selector == 0 || d.receiver.length == 0) revert DstNotConfigured();

        address tokenIn = abi.decode(m.token, (address));
        CCIPClient.EVMTokenAmount ;
        toks[0] = CCIPClient.EVMTokenAmount({token: tokenIn, amount: m.amountIn});

        CCIPClient.EVM2AnyMessage memory msg2 = CCIPClient.EVM2AnyMessage({
            receiver: d.receiver,
            data: orpMessage,
            tokenAmounts: toks,
            feeToken: feeToken,
            extraArgs: ""
        });

        return IRouterClient(ccipRouter).getFee(d.selector, msg2);
    }

    function send(bytes calldata orpMessage) external payable returns (bytes32 msgId) {
        if (address(ccipRouter) == address(0)) revert RouterNotSet();
        ORPMessage memory m = abi.decode(orpMessage, (ORPMessage));
        DestConfig memory d = dest[m.dstChainId];
        if (d.selector == 0 || d.receiver.length == 0) revert DstNotConfigured();

        address tokenIn = abi.decode(m.token, (address));
        IERC20 t = IERC20(tokenIn);

        t.safeTransferFrom(msg.sender, address(this), m.amountIn);
        t.safeApprove(address(ccipRouter), 0);
        t.safeApprove(address(ccipRouter), m.amountIn);

        CCIPClient.EVMTokenAmount ;
        toks[0] = CCIPClient.EVMTokenAmount({token: tokenIn, amount: m.amountIn});

        CCIPClient.EVM2AnyMessage memory msg2 = CCIPClient.EVM2AnyMessage({
            receiver: d.receiver,
            data: orpMessage,
            tokenAmounts: toks,
            feeToken: feeToken,
            extraArgs: ""
        });

        msgId = IRouterClient(ccipRouter).ccipSend{value: msg.value}(d.selector, msg2);
        emit Sent(msgId, d.selector, tokenIn, m.amountIn, d.receiver);
    }

    /*//////////////////////////////////////////////////////////////
                           CCIP RECEIVE
    //////////////////////////////////////////////////////////////*/
    function ccipReceive(CCIPClient.Any2EVMMessage calldata message) external override {
        if (msg.sender != address(ccipRouter)) revert RouterNotSet();
        ORPMessage memory m = abi.decode(message.data, (ORPMessage));
        DestConfig memory d = dest[m.dstChainId];
        if (d.routerCore == address(0)) revert DstNotConfigured();

        for (uint256 i = 0; i < message.destTokenAmounts.length; i++) {
            CCIPClient.EVMTokenAmount calldata ta = message.destTokenAmounts[i];
            IERC20(ta.token).safeTransfer(d.routerCore, ta.amount);
        }

        IRouterCore(d.routerCore).onORPMessage(message.messageId, m, d.dex, d.dexData);
        emit Delivered(message.messageId, message.sourceChainSelector, d.routerCore, address(0), 0, abi.decode(m.receiver, (address)));
    }

    receive() external payable {}
}
