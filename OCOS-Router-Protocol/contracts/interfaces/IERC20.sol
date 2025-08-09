// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/**
 * @title IERC20
 * @notice EIP-20 (ERC-20) standard token interface.
 *
 * @dev This interface strictly follows the EIP-20 specification:
 * - transfer/transferFrom/approve functions MUST return a boolean
 * - Transfer and Approval events MUST be emitted on state changes
 * - totalSupply, balanceOf, allowance functions MUST be view
 *
 * Usage Notes:
 * - Some tokens in the wild return `false` instead of reverting on failure.
 *   Always check the returned boolean.
 * - For safer token operations, use wrapper libraries (e.g., SafeERC20).
 */
interface IERC20 {
    /**
     * @notice Returns the total token supply in circulation.
     */
    function totalSupply() external view returns (uint256);

    /**
     * @notice Returns the account balance of a given address.
     * @param account Address to query the balance for.
     */
    function balanceOf(address account) external view returns (uint256);

    /**
     * @notice Transfers `amount` tokens to the specified address `to`.
     * @dev MUST emit a Transfer event on success.
     * @param to The recipient address.
     * @param amount Number of tokens to transfer.
     * @return success True if the operation succeeded.
     */
    function transfer(address to, uint256 amount) external returns (bool success);

    /**
     * @notice Returns the remaining number of tokens that `spender` is allowed
     *         to spend on behalf of `owner` through transferFrom.
     * @param owner Token owner's address.
     * @param spender Address authorized to spend.
     */
    function allowance(address owner, address spender) external view returns (uint256);

    /**
     * @notice Approves `spender` to withdraw from your account multiple times,
     *         up to the `amount` value.
     * @dev MUST emit an Approval event on success.
     *      To mitigate the race condition issue described in EIP-20, it’s recommended
     *      to first reduce the spender’s allowance to 0 before setting a new value.
     * @param spender Address authorized to spend.
     * @param amount Max amount they can spend.
     * @return success True if the operation succeeded.
     */
    function approve(address spender, uint256 amount) external returns (bool success);

    /**
     * @notice Transfers `amount` tokens from address `from` to address `to` using
     *         the allowance mechanism.
     * @dev MUST emit a Transfer event on success.
     *      The caller must have allowance for `from`'s tokens of at least `amount`.
     * @param from Address sending the tokens.
     * @param to Address receiving the tokens.
     * @param amount Number of tokens to transfer.
     * @return success True if the operation succeeded.
     */
    function transferFrom(address from, address to, uint256 amount) external returns (bool success);

    /**
     * @notice Emitted when `value` tokens are moved from one account (`from`) to another (`to`).
     * @dev Note: `from` may be the zero address for minting,
     *            `to` may be the zero address for burning.
     */
    event Transfer(address indexed from, address indexed to, uint256 value);

    /**
     * @notice Emitted when the allowance of a `spender` for an `owner` is set by a call to approve.
     */
    event Approval(address indexed owner, address indexed spender, uint256 value);
}

/**
 * @title IERC20Metadata
 * @notice Optional ERC-20 extension for token metadata.
 * @dev Widely adopted by most token implementations.
 */
interface IERC20Metadata is IERC20 {
    /**
     * @notice Returns the human-readable name of the token.
     */
    function name() external view returns (string memory);

    /**
     * @notice Returns the symbol of the token, usually a shorter version of the name.
     */
    function symbol() external view returns (string memory);

    /**
     * @notice Returns the number of decimal places used to get its user representation.
     * @dev For example, `decimals` = 6 means balances are in millionths of the token unit.
     */
    function decimals() external view returns (uint8);
}

/* -----------------------------------------------------------------------
   Notes:
   - IERC20Permit (EIP-2612) is not included here. If needed, define it
     as a separate interface file (IERC20Permit.sol).
   - This is purely an interface; no storage or implementation logic is present.
------------------------------------------------------------------------ */
