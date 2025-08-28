// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

/**
 * @title BTCReserveRegistry
 * @notice Public registry of BTC reserve addresses (as keccak256 of the UTF-8 address string).
 *         This is informational for EVM consumers; PoR totals come from PoRGuard oracle updates.
 */
contract BTCReserveRegistry is AccessControl {
    bytes32 public constant MANAGER_ROLE = keccak256("MANAGER_ROLE");

    mapping(bytes32 => bool) public isListed;   // keccak256(bytes(addr)) => listed?
    bytes32[] public entries;

    event AddressAdded(bytes32 indexed key, string addr);
    event AddressRemoved(bytes32 indexed key, string addr);

    constructor() {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(MANAGER_ROLE, msg.sender);
    }

    function add(string calldata addr) external onlyRole(MANAGER_ROLE) {
        bytes32 key = keccak256(bytes(addr));
        require(!isListed[key], "already listed");
        isListed[key] = true;
        entries.push(key);
        emit AddressAdded(key, addr);
    }

    function remove(string calldata addr) external onlyRole(MANAGER_ROLE) {
        bytes32 key = keccak256(bytes(addr));
        require(isListed[key], "not listed");
        isListed[key] = false;
        emit AddressRemoved(key, addr);
        // (Keeping key in `entries` for audit trace; clients must check isListed[key].)
    }

    function size() external view returns (uint256) {
        return entries.length;
    }

    function listed(bytes32 key) external view returns (bool) {
        return isListed[key];
    }

    function listedByString(string calldata addr) external view returns (bool) {
        return isListed[keccak256(bytes(addr))];
    }
}
