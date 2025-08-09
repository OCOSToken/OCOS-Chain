// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/**
 * @title UniswapV3Adapter
 * @notice IDEXAdapter implementasiyası (Uniswap V3 / Pancake V3 / Algebra V3-abi uyğun routerlər üçün).
 *
 * Xüsusiyyətlər:
 * - Single-hop: ISwapRouter.exactInputSingle(...)
 * - Multi-hop:  ISwapRouter.exactInput(path, ...)
 * - Rejim seçimi və parametrlər `dexData` içində abi.encode(...) ilə ötürülür.
 *
 * Qeyd:
 * - Uniswap V3-də "fee-on-transfer" tokenlər üçün xüsusi supporting funksiya yoxdur.
 *   Belə tokenlərlə swap router səviyyəsində uğursuz ola bilər.
 */

/// ====== Minimal ERC20 ======
interface IERC20 {
    function approve(address, uint256) external returns (bool);
    function transfer(address, uint256) external returns (bool);
    function transferFrom(address, address, uint256) external returns (bool);
    function balanceOf(address) external view returns (uint256);
}

/// ====== IDEXAdapter (RouterCore ilə eyni interfeys) ======
interface IDEXAdapter {
    function swapExactIn(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minOut,
        bytes calldata dexData
    ) external returns (uint256 amountOut);
}

/// ====== Uniswap V3 ISwapRouter ABI ======
interface ISwapRouter {
    struct ExactInputSingleParams {
        address tokenIn;
        address tokenOut;
        uint24  fee;
        address recipient;
        uint256 deadline;
        uint256 amountIn;
        uint256 amountOutMinimum;
        uint160 sqrtPriceLimitX96; // 0 → no limit
    }

    struct ExactInputParams {
        bytes   path;               // tokenIn|fee|tokenMid|fee|...|tokenOut
        address recipient;
        uint256 deadline;
        uint256 amountIn;
        uint256 amountOutMinimum;
    }

    function exactInputSingle(ExactInputSingleParams calldata params)
        external
        payable
        returns (uint256 amountOut);

    function exactInput(ExactInputParams calldata params)
        external
        payable
        returns (uint256 amountOut);
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

contract UniswapV3Adapter is IDEXAdapter {
    using SafeTransferLib for IERC20;

    /// @dev Uniswap V3 uyğun router (məs.: UniswapV3Router, PancakeV3Router).
    ISwapRouter public immutable router;

    event V3SwapExecuted(
        address indexed caller,
        address indexed tokenIn,
        address indexed tokenOut,
        uint256 amountIn,
        uint256 amountOut,
        bool multiHop,
        uint256 deadline
    );

    error RouterZero();
    error InvalidSingle();
    error InvalidPath();
    error DeadlineZero();

    /**
     * @param _router Uniswap V3 uyğun router ünvanı
     */
    constructor(address _router) {
        if (_router == address(0)) revert RouterZero();
        router = ISwapRouter(_router);
    }

    /**
     * @notice Uniswap V3 swap (single və ya multi-hop).
     * @param tokenIn  Giriş tokeni (Single-hop üçün router.param da eynidir; multi-hop üçün path[0] ilə eyni olmalı)
     * @param tokenOut Çıxış tokeni (Single-hop üçün router.param da eynidir; multi-hop üçün path son tokeni ilə eyni olmalı)
     * @param amountIn Giriş məbləği (RouterCore bu adapterə transferFrom edir)
     * @param minOut   Minimal qəbul edilən çıxış (slippage qoruması)
     * @param dexData  Rejimə görə abi.encode(...):
     *
     *   SINGLE-HOP FORMAT:
     *     abi.encode(
     *       false,              // multiHop = false
     *       uint24 fee,         // pool fee (məs.: 500, 3000, 10000)
     *       uint160 sqrtLimit,  // 0 → limit yoxdur
     *       uint256 deadline    // 0 deyilsə, istifadə olunur; 0-dırsa, adapter default 15 dəq əlavə edər
     *     )
     *
     *   MULTI-HOP FORMAT:
     *     abi.encode(
     *       true,               // multiHop = true
     *       bytes path,         // encoded path: tokenIn(20)|fee(3)|tokenMid(20)|fee(3)|...|tokenOut(20)
     *       uint256 deadline    // 0 deyilsə, istifadə olunur; 0-dırsa, adapter default 15 dəq əlavə edər
     *     )
     */
    function swapExactIn(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minOut,
        bytes calldata dexData
    ) external override returns (uint256 amountOut) {
        (bool multiHop) = abi.decode(dexData[:32], (bool)); // ilk slotu oxu

        if (multiHop) {
            // decode: (bool, bytes, uint256)
            (, bytes memory path, uint256 deadline) = abi.decode(dexData, (bool, bytes, uint256));
            if (deadline == 0) {
                deadline = block.timestamp + 900; // 15 dəqiqə
            }

            // Yol yoxlaması: path-in ilk və son tokeni tokenIn/tokenOut-la uyğun olmalıdır
            (address pathTokenIn, address pathTokenOut) = _extractEndpointsFromPath(path);
            if (pathTokenIn != tokenIn || pathTokenOut != tokenOut) revert InvalidPath();

            // RouterCore → Adapter token köçürür, buradan router çəkəcək
            IERC20(tokenIn).safeTransferFrom(msg.sender, address(this), amountIn);
            IERC20(tokenIn).safeApprove(address(router), 0);
            IERC20(tokenIn).safeApprove(address(router), amountIn);

            amountOut = router.exactInput(
                ISwapRouter.ExactInputParams({
                    path: path,
                    recipient: address(this),
                    deadline: deadline,
                    amountIn: amountIn,
                    amountOutMinimum: minOut
                })
            );

            IERC20(tokenOut).safeTransfer(msg.sender, amountOut);
            emit V3SwapExecuted(msg.sender, tokenIn, tokenOut, amountIn, amountOut, true, deadline);
        } else {
            // decode: (bool, uint24, uint160, uint256)
            (, uint24 fee, uint160 sqrtLimit, uint256 deadline) = abi.decode(dexData, (bool, uint24, uint160, uint256));
            if (deadline == 0) {
                deadline = block.timestamp + 900;
            }
            if (tokenIn == address(0) || tokenOut == address(0)) revert InvalidSingle();

            IERC20(tokenIn).safeTransferFrom(msg.sender, address(this), amountIn);
            IERC20(tokenIn).safeApprove(address(router), 0);
            IERC20(tokenIn).safeApprove(address(router), amountIn);

            amountOut = router.exactInputSingle(
                ISwapRouter.ExactInputSingleParams({
                    tokenIn: tokenIn,
                    tokenOut: tokenOut,
                    fee: fee,
                    recipient: address(this),
                    deadline: deadline,
                    amountIn: amountIn,
                    amountOutMinimum: minOut,
                    sqrtPriceLimitX96: sqrtLimit
                })
            );

            IERC20(tokenOut).safeTransfer(msg.sender, amountOut);
            emit V3SwapExecuted(msg.sender, tokenIn, tokenOut, amountIn, amountOut, false, deadline);
        }
    }

    /* =========================
            INTERNAL
       ========================= */

    /// @dev UniswapV3 path-ın (bytes) başlanğıc və son token ünvanlarını çıxarır.
    /// path formatı: token(20) | fee(3) | token(20) | fee(3) | ... | token(20)
    function _extractEndpointsFromPath(bytes memory path) internal pure returns (address tokenA, address tokenB) {
        uint256 len = path.length;
        // ən azı 20 + 3 + 20 = 43 bayt olmalıdır (bir hop)
        if (len < 43) revert InvalidPath();

        // ilk token (ilk 20 bayt)
        tokenA = _bytesToAddress(path, 0);

        // son token (son 20 bayt), arada (len - 20) offset
        tokenB = _bytesToAddress(path, len - 20);
    }

    function _bytesToAddress(bytes memory data, uint256 start) internal pure returns (address addr) {
        require(data.length >= start + 20, "PATH_OOB");
        assembly {
            addr := shr(96, mload(add(add(data, 0x20), start)))
        }
    }
}
