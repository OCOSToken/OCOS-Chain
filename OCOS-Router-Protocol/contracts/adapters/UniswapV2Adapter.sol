// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/**
 * @title UniswapV2Adapter
 * @notice IDEXAdapter implementation for Uniswap V2-style routers (Pancake V2, Sushi, etc.)
 *
 * DEX assumptions:
 * - Router exposes both:
 *   - swapExactTokensForTokens(uint amountIn, uint amountOutMin, address[] path, address to, uint deadline)
 *   - swapExactTokensForTokensSupportingFeeOnTransferTokens(uint amountIn, uint amountOutMin, address[] path, address to, uint deadline)
 *
 * Call context:
 * - This adapter is called by RouterCore, which has already:
 *   - Pulled tokens from the user
 *   - Approved *this adapter* to spend `tokenIn` exactly `amountIn`
 *
 * Security:
 * - Pulls tokens from msg.sender (RouterCore) with transferFrom
 * - Approves router for exact amount (resets to 0 first to avoid allowance race)
 * - For fee-on-transfer path, computes amountOut via balance delta and enforces minOut
 */

/// ====== Minimal ERC20 ======
interface IERC20 {
    function approve(address, uint256) external returns (bool);
    function transfer(address, uint256) external returns (bool);
    function transferFrom(address, address, uint256) external returns (bool);
    function balanceOf(address) external view returns (uint256);
}

/// ====== Router ABI (Uniswap V2-compatible) ======
interface IUniswapV2Router02 {
    function swapExactTokensForTokens(
        uint amountIn,
        uint amountOutMin,
        address[] calldata path,
        address to,
        uint deadline
    ) external returns (uint[] memory amounts);

    function swapExactTokensForTokensSupportingFeeOnTransferTokens(
        uint amountIn,
        uint amountOutMin,
        address[] calldata path,
        address to,
        uint deadline
    ) external;
}

/// ====== IDEXAdapter (as in contracts/interfaces/IDEXAdapter.sol) ======
interface IDEXAdapter {
    function swapExactIn(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minOut,
        bytes calldata dexData
    ) external returns (uint256 amountOut);
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

contract UniswapV2Adapter is IDEXAdapter {
    using SafeTransferLib for IERC20;

    /// @dev Immutable DEX router (UniswapV2-compatible)
    IUniswapV2Router02 public immutable router;

    /// @notice Emitted on each swap
    event SwapExecuted(address indexed caller, address indexed tokenIn, address indexed tokenOut, uint256 amountIn, uint256 amountOut, bool feeOnTransfer, uint256 deadline);

    error InvalidPath();
    error MinOutNotMet();
    error RouterZero();

    /**
     * @param _router UniswapV2-compatible router address (e.g., PancakeRouterV2 on BSC)
     */
    constructor(address _router) {
        if (_router == address(0)) revert RouterZero();
        router = IUniswapV2Router02(_router);
    }

    /**
     * @notice Perform swap on Uniswap V2-style router.
     * @param tokenIn  ERC20 input token (must equal path[0])
     * @param tokenOut ERC20 output token (must equal path[last])
     * @param amountIn Input amount (RouterCore has approved this adapter for amountIn)
     * @param minOut   Minimum acceptable output (slippage guard)
     * @param dexData  Encoded params:
     *                 abi.encode(
     *                   address[] path,          // swap path; length >= 2
     *                   bool feeOnTransfer,      // use supportingFeeOnTransfer function
     *                   uint256 deadline         // router deadline; if 0, adapter uses block.timestamp + 900
     *                 )
     *
     * @return amountOut Output token amount received by this adapter (then transferred back to msg.sender)
     */
    function swapExactIn(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minOut,
        bytes calldata dexData
    ) external override returns (uint256 amountOut) {
        (address[] memory path, bool feeOnTransfer, uint256 deadline) = _decodeDexData(dexData);

        if (path.length < 2 || path[0] != tokenIn || path[path.length - 1] != tokenOut) {
            revert InvalidPath();
        }
        if (deadline == 0) {
            // default 15 minutes window if not provided
            deadline = block.timestamp + 900;
        }

        IERC20 inT  = IERC20(tokenIn);
        IERC20 outT = IERC20(tokenOut);

        // Pull tokens from RouterCore → adapter
        inT.safeTransferFrom(msg.sender, address(this), amountIn);

        // Approve router for exact amount
        inT.safeApprove(address(router), 0);
        inT.safeApprove(address(router), amountIn);

        if (feeOnTransfer) {
            // Measure balance before/after because the supporting function doesn't return amounts
            uint256 beforeBal = outT.balanceOf(address(this));
            router.swapExactTokensForTokensSupportingFeeOnTransferTokens(
                amountIn,
                minOut,
                path,
                address(this),
                deadline
            );
            uint256 afterBal = outT.balanceOf(address(this));
            amountOut = afterBal - beforeBal;

            // Enforce minOut (router also enforces via param, but we double-check)
            if (amountOut < minOut) revert MinOutNotMet();
        } else {
            uint[] memory amounts = router.swapExactTokensForTokens(
                amountIn,
                minOut,
                path,
                address(this),
                deadline
            );
            // amounts array is path-length; last element is output
            amountOut = amounts[amounts.length - 1];
        }

        // Send the output back to caller (RouterCore), which will forward to end receiver
        outT.safeTransfer(msg.sender, amountOut);

        emit SwapExecuted(msg.sender, tokenIn, tokenOut, amountIn, amountOut, feeOnTransfer, deadline);
    }

    /* =========================
            INTERNAL
       ========================= */

    function _decodeDexData(bytes calldata dexData)
        internal
        pure
        returns (address[] memory path, bool feeOnTransfer, uint256 deadline)
    {
        // Primary format
        if (dexData.length > 0) {
            (path, feeOnTransfer, deadline) = abi.decode(dexData, (address[], bool, uint256));
        } else {
            // Fallback: no data → single-hop path [tokenIn, tokenOut], no fee-on-transfer, default deadline
            // NOTE: This fallback cannot construct path without inputs; callers should always pass dexData.
            revert InvalidPath();
        }
    }
}
