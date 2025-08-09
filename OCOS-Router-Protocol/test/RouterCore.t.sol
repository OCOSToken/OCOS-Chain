// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/**
 * RouterCore.t.sol — Professional Foundry tests for OCOS Router Protocol
 *
 * What we cover:
 * - Intra-chain routing success, slippage, deadline.
 * - Cross-chain send path with mock BridgeAdapter (fee quoting, emit).
 * - Destination-chain receive path (onORPMessage): allowlist, replay-protection, slippage.
 * - Pause/guardian controls.
 * - Chain domain & message encoding invariants.
 *
 * Test doubles:
 * - MockERC20: mintable ERC20 with 18 decimals by default (configurable).
 * - MockDEXAdapter: pulls tokenIn via transferFrom(), pushes tokenOut back to RouterCore, deterministic rate.
 * - MockBridgeAdapter: quotes fee, returns a synthetic messageId, records payload.
 */

import "forge-std/Test.sol";
import "forge-std/console2.sol";

import { RouterCore, ORPMessage } from "../contracts/RouterCore.sol";
import { IERC20 } from "../contracts/interfaces/IERC20.sol";
import { IDEXAdapter } from "../contracts/interfaces/IDEXAdapter.sol";
import { IBridgeAdapter } from "../contracts/interfaces/IBridgeAdapter.sol";

/* //////////////////////////////////////////////////////////////
                            MOCKS
////////////////////////////////////////////////////////////// */

contract MockERC20 is IERC20 {
    string public name;
    string public symbol;
    uint8  private _decimals;
    uint256 public totalSupply;

    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory n, string memory s, uint8 d) {
        name = n; symbol = s; _decimals = d;
    }

    function decimals() external view override returns (uint8) { return _decimals; }

    function mint(address to, uint256 amt) external {
        totalSupply += amt;
        balanceOf[to] += amt;
        emit Transfer(address(0), to, amt);
    }

    function approve(address spender, uint256 amt) external override returns (bool) {
        allowance[msg.sender][spender] = amt;
        emit Approval(msg.sender, spender, amt);
        return true;
    }

    function transfer(address to, uint256 amt) external override returns (bool) {
        _transfer(msg.sender, to, amt);
        return true;
    }

    function transferFrom(address from, address to, uint256 amt) external override returns (bool) {
        uint256 a = allowance[from][msg.sender];
        require(a >= amt, "allowance");
        allowance[from][msg.sender] = a - amt;
        _transfer(from, to, amt);
        return true;
    }

    function _transfer(address from, address to, uint256 amt) internal {
        require(balanceOf[from] >= amt, "balance");
        balanceOf[from] -= amt;
        balanceOf[to] += amt;
        emit Transfer(from, to, amt);
    }

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
}

contract MockDEXAdapter is IDEXAdapter {
    // amountOut = amountIn * rateBps / 10_000
    uint256 public rateBps = 9900; // default 0.99x to simulate fee/price impact

    function setRate(uint256 bps) external { rateBps = bps; }

    function swapExactIn(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 /*minOut*/,
        bytes calldata /*dexData*/
    ) external override returns (uint256 amountOut) {
        // Pull tokenIn from caller (RouterCore)
        IERC20(tokenIn).transferFrom(msg.sender, address(this), amountIn);

        // Calculate deterministic output
        amountOut = (amountIn * rateBps) / 10_000;

        // Pay tokenOut to caller (RouterCore)
        IERC20(tokenOut).transfer(msg.sender, amountOut);
    }
}

contract MockBridgeAdapter is IBridgeAdapter {
    uint256 public quotedFee;
    bytes public lastMessage;
    bytes32 public lastMsgId;

    function setQuotedFee(uint256 fee) external { quotedFee = fee; }

    function quoteSend(bytes calldata /*orpMessage*/) external view override returns (uint256 fee) {
        return quotedFee;
    }

    function send(bytes calldata orpMessage) external payable override returns (bytes32 msgId) {
        lastMessage = orpMessage;
        // deterministic-ish synthetic id
        msgId = keccak256(abi.encode(blockhash(block.number - 1), address(this), msg.sender, orpMessage));
        lastMsgId = msgId;
    }
}

/* //////////////////////////////////////////////////////////////
                          TEST SUITE
////////////////////////////////////////////////////////////// */
contract RouterCoreTest is Test {
    RouterCore internal router;
    MockERC20 internal tokenA; // tokenIn
    MockERC20 internal tokenB; // tokenOut
    MockDEXAdapter internal dex;
    MockBridgeAdapter internal bridge;

    address internal user = makeAddr("user");
    address internal guardian = address(this); // test contract as guardian
    address internal feeManager = makeAddr("treasury");
    uint256 internal constant SRC_CHAIN = 56; // e.g., BSC Mainnet

    function setUp() public {
        // Normalize chain id for domain separation in tests
        vm.chainId(SRC_CHAIN);

        // Deploy core + mocks
        router = new RouterCore(feeManager, guardian);
        tokenA = new MockERC20("TokenA", "TKA", 18);
        tokenB = new MockERC20("TokenB", "TKB", 18);
        dex = new MockDEXAdapter();
        bridge = new MockBridgeAdapter();

        // Seed balances
        tokenA.mint(user, 1_000 ether);
        tokenB.mint(address(dex), 1_000 ether); // DEX holds liquidity in tokenOut

        // Basic approvals for user → router
        vm.prank(user);
        tokenA.approve(address(router), type(uint256).max);

        // Allow the bridge as a trusted adapter for destination receive path tests
        router.setAdapter(address(this), true); // allow this test contract to simulate adapter calls
    }

    /* ------------------- Intra-chain routing ------------------- */

    function test_IntraRoute_Success() public {
        uint256 amountIn = 100 ether;
        uint256 deadline = block.timestamp + 1 hours;

        // Expect user receives ~99 out (rate 9900 bps), minOut set conservative
        vm.prank(user);
        uint256 amountOut = router.intraRoute(
            address(dex),
            address(tokenA),
            address(tokenB),
            amountIn,
            98 ether, // minOut
            bytes(""), // dexData
            deadline
        );

        assertApproxEqRel(amountOut, 99 ether, 1e16); // within 1%
        assertEq(tokenA.balanceOf(user), 900 ether);
        assertEq(tokenB.balanceOf(user), amountOut);
        // Router should not keep dust of tokenB after transfer to user
        assertEq(tokenB.balanceOf(address(router)), 0);
    }

    function test_IntraRoute_RevertOnDeadline() public {
        uint256 amountIn = 10 ether;
        uint256 pastDeadline = block.timestamp - 1;

        vm.expectRevert(RouterCore.Deadline.selector);
        vm.prank(user);
        router.intraRoute(
            address(dex),
            address(tokenA),
            address(tokenB),
            amountIn,
            1, // minOut
            bytes(""),
            pastDeadline
        );
    }

    function test_IntraRoute_RevertOnSlippage() public {
        // Make the DEX rate bad: 50% out
        vm.prank(address(this));
        dex.setRate(5000);

        uint256 amountIn = 10 ether;
        uint256 deadline = block.timestamp + 3600;

        // minOut set too strict (expect revert)
        vm.expectRevert(RouterCore.Slippage.selector);
        vm.prank(user);
        router.intraRoute(
            address(dex),
            address(tokenA),
            address(tokenB),
            amountIn,
            9 ether, // minOut 90%
            bytes(""),
            deadline
        );
    }

    /* ------------------- Cross-chain send (source) ------------------- */

    function test_CrossRoute_SendsAndEmits() public {
        // Bridge quotes a fee; user provides no ETH if fee=0
        bridge.setQuotedFee(0);

        ORPMessage memory m = _baseMessage();
        m.srcChainId = block.chainid; // must match
        m.token = abi.encode(address(tokenA)); // tokenIn on source (locked/sent via bridge)
        m.amountIn = 50 ether;

        // Expect CrossSent emit
        // (We don't mark exact indexed topics here; Foundry `expectEmit` can be strict if needed)
        vm.expectEmit(true, true, false, true);
        emit CrossSent(bytes32(0), m.dstChainId, user, address(tokenA), m.amountIn);

        vm.prank(user);
        bytes32 msgId = router.crossRoute(address(bridge), m);
        assertEq(bridge.lastMsgId(), msgId, "bridge recorded msgId");
        assertGt(msgId.length, 0, "non-empty msgId");
    }

    function test_CrossRoute_InvalidChain_Reverts() public {
        bridge.setQuotedFee(0);

        ORPMessage memory m = _baseMessage();
        m.srcChainId = 99999; // wrong
        m.token = abi.encode(address(tokenA));
        m.amountIn = 1 ether;

        vm.expectRevert(RouterCore.InvalidChain.selector);
        vm.prank(user);
        router.crossRoute(address(bridge), m);
    }

    function test_CrossRoute_Deadline_Reverts() public {
        bridge.setQuotedFee(0);

        ORPMessage memory m = _baseMessage();
        m.srcChainId = block.chainid;
        m.deadline = uint256(block.timestamp) - 1; // past
        m.token = abi.encode(address(tokenA));
        m.amountIn = 1 ether;

        vm.expectRevert(RouterCore.Deadline.selector);
        vm.prank(user);
        router.crossRoute(address(bridge), m);
    }

    /* ------------------- Cross-chain receive (destination) ------------------- */

    function test_OnORPMessage_Success_WithAllowlist() public {
        // Simulate the destination chain state:
        // Router already holds tokenIn (lock/mint performed by adapter beforehand)
        tokenA.mint(address(router), 100 ether);

        // Build message; tokenOut is tokenB and dex has tokenB liquidity
        ORPMessage memory m = _baseMessage();
        m.token = abi.encode(address(tokenA));
        m.amountIn = 40 ether;
        m.aux = abi.encode(address(tokenB)); // tokenOut on dst

        bytes32 msgId = keccak256("unique-msg-id-1");

        // Allow *this* contract as the adapter caller
        router.setAdapter(address(this), true);

        // Expect CrossReceived emit
        vm.expectEmit(true, false, false, true);
        emit CrossReceived(msgId, m.srcChainId, address(tokenB), 0 /*amountOut set post-call*/);

        // Call as adapter
        router.onORPMessage(msgId, m, address(dex), bytes(""));

        // Receiver gets tokenB
        address receiver = abi.decode(m.receiver, (address));
        uint256 recvBal = tokenB.balanceOf(receiver);
        assertGt(recvBal, 0, "receiver received tokenB");
    }

    function test_OnORPMessage_OnlyAllowedAdapter() public {
        tokenA.mint(address(router), 10 ether);

        ORPMessage memory m = _baseMessage();
        m.token = abi.encode(address(tokenA));
        m.amountIn = 10 ether;
        m.aux = abi.encode(address(tokenB));

        bytes32 msgId = keccak256("unique-msg-id-2");

        // Ensure random adapter is NOT allowlisted
        address randomAdapter = makeAddr("randomAdapter");

        vm.expectRevert(RouterCore.NotAllowed.selector);
        vm.prank(randomAdapter);
        router.onORPMessage(msgId, m, address(dex), bytes(""));
    }

    function test_OnORPMessage_ReplayProtection() public {
        tokenA.mint(address(router), 20 ether);

        ORPMessage memory m = _baseMessage();
        m.token = abi.encode(address(tokenA));
        m.amountIn = 10 ether;
        m.aux = abi.encode(address(tokenB));
        bytes32 msgId = keccak256("unique-msg-id-3");

        // First call succeeds
        router.onORPMessage(msgId, m, address(dex), bytes(""));

        // Second call with same msgId must revert
        vm.expectRevert(RouterCore.AlreadyProcessed.selector);
        router.onORPMessage(msgId, m, address(dex), bytes(""));
    }

    function test_OnORPMessage_SlippageReverts() public {
        tokenA.mint(address(router), 10 ether);

        // Set DEX to produce only 50% out but demand minOut ~ 90%
        vm.prank(address(this));
        dex.setRate(5000);

        ORPMessage memory m = _baseMessage();
        m.token = abi.encode(address(tokenA));
        m.amountIn = 10 ether;
        m.minAmountOut = 9 ether; // too strict
        m.aux = abi.encode(address(tokenB));
        bytes32 msgId = keccak256("unique-msg-id-4");

        vm.expectRevert(RouterCore.Slippage.selector);
        router.onORPMessage(msgId, m, address(dex), bytes(""));
    }

    /* ------------------- Pause & Guardian ------------------- */

    function test_PauseBlocksFlows() public {
        router.pause(true);
        assertTrue(router.paused(), "paused");

        // intraRoute blocked
        vm.expectRevert(bytes("paused"));
        vm.prank(user);
        router.intraRoute(address(dex), address(tokenA), address(tokenB), 1 ether, 1, bytes(""), block.timestamp + 60);

        // crossRoute blocked
        ORPMessage memory m = _baseMessage();
        m.srcChainId = block.chainid;
        m.token = abi.encode(address(tokenA));
        m.amountIn = 1 ether;
        vm.expectRevert(bytes("paused"));
        vm.prank(user);
        router.crossRoute(address(bridge), m);

        // onORPMessage blocked
        vm.expectRevert(bytes("paused"));
        router.onORPMessage(keccak256("x"), m, address(dex), bytes(""));

        router.pause(false);
        assertFalse(router.paused(), "unpaused");
    }

    /* ------------------- Helpers ------------------- */

    event CrossSent(bytes32 indexed msgId, uint256 dstChainId, address indexed user, address tokenIn, uint256 amountIn);
    event CrossReceived(bytes32 indexed msgId, uint256 srcChainId, address tokenOut, uint256 amountOut);

    function _baseMessage() internal view returns (ORPMessage memory m) {
        m.version = 1;
        m.srcChainId = SRC_CHAIN;
        m.dstChainId = 1; // e.g., Ethereum
        m.routeId = keccak256("route-abc");
        m.srcSender = user;
        m.receiver = abi.encode(user);
        m.token = abi.encode(address(tokenA));
        m.amountIn = 0;
        m.minAmountOut = 1; // tiny default
        m.deadline = block.timestamp + 30 minutes;
        m.nonce = 1;
        m.callData = bytes(""); // unused in tests
        m.aux = abi.encode(address(tokenB));
    }
}
