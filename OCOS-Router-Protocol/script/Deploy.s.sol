// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/**
 * Deploy.s.sol — OCOS Router Protocol (ORP)
 *
 * What this script does:
 *  - Broadcasts with your EOA (the one you pass to `forge script ... --private-key`)
 *  - Deploys RouterCore with feeManager = guardian = your EOA
 *  - (Optional) Deploys CCIPAdapter if env var CCIP_ROUTER is provided
 *  - Allowlists the newly deployed adapter(s) on RouterCore
 *  - Writes a JSON file under ./deployments/<chainId>.json with deployed addresses
 *
 * Usage examples:
 *  forge script script/Deploy.s.sol \
 *    --rpc-url $RPC --private-key $PK --broadcast
 *
 *  With CCIP:
 *  CCIP_ROUTER=0x... \
 *  forge script script/Deploy.s.sol \
 *    --rpc-url $RPC --private-key $PK --broadcast
 */

import "forge-std/Script.sol";
import "forge-std/console2.sol";

import {RouterCore} from "../contracts/RouterCore.sol";
import {IBridgeAdapter} from "../contracts/interfaces/IBridgeAdapter.sol";
import {IERC20} from "../contracts/interfaces/IERC20.sol";

/* Optional adapter */
import {CCIPAdapter} from "../contracts/adapters/CCIPAdapter.sol";

contract Deploy is Script {
    /* ------------------------------------------------------------ */
    /*                       CONFIG / CONSTANTS                     */
    /* ------------------------------------------------------------ */

    // Where to write the deployment file (ensure the folder exists in your repo)
    string internal constant DEPLOY_DIR = "./deployments";

    /* ------------------------------------------------------------ */
    /*                           ENTRYPOINT                         */
    /* ------------------------------------------------------------ */

    function run() external {
        // Read chainId before broadcast for logging
        uint256 chainId = block.chainid;

        // Start broadcasting with the PK you provide to forge
        vm.startBroadcast();

        address deployer = msg.sender; // your EOA
        console2.log("Deployer:", deployer);
        console2.log("ChainId :", chainId);

        // 1) Deploy RouterCore (feeManager = guardian = deployer)
        RouterCore router = new RouterCore(deployer, deployer);
        console2.log("RouterCore deployed at:", address(router));

        // 2) (Optional) Deploy CCIPAdapter if env CCIP_ROUTER is provided
        address maybeCcipRouter = _maybeEnvAddress("CCIP_ROUTER");
        address ccipAdapterAddr = address(0);

        if (maybeCcipRouter != address(0)) {
            CCIPAdapter ccip = new CCIPAdapter(maybeCcipRouter);
            ccipAdapterAddr = address(ccip);
            console2.log("CCIPAdapter deployed at:", ccipAdapterAddr);
            // Allowlist adapter on RouterCore (guardian = deployer)
            router.setAdapter(ccipAdapterAddr, true);
            console2.log("CCIPAdapter allowlisted on RouterCore");
        } else {
            console2.log("CCIP_ROUTER not provided — skipping CCIPAdapter deploy");
        }

        // 3) (Optional) Allowlist any pre-existing adapters from env JSON:
        //    ADAPTERS_JSON='["0xAdapter1","0xAdapter2"]'
        //    This is useful if you already have adapters deployed.
        address[] memory extraAdapters = _maybeEnvAddressArray("ADAPTERS_JSON");
        if (extraAdapters.length > 0) {
            for (uint256 i = 0; i < extraAdapters.length; i++) {
                if (extraAdapters[i] == address(0)) continue;
                router.setAdapter(extraAdapters[i], true);
                console2.log("Allowlisted adapter:", extraAdapters[i]);
            }
        }

        vm.stopBroadcast();

        // 4) Persist deployment data to ./deployments/<chainId>.json
        _writeDeployJson(chainId, address(router), ccipAdapterAddr);
    }

    /* ------------------------------------------------------------ */
    /*                         HELPER: JSON                         */
    /* ------------------------------------------------------------ */

    function _writeDeployJson(
        uint256 chainId,
        address router,
        address ccipAdapter
    ) internal {
        // Root label for serialization
        string memory root = "deploy";

        // serialize addresses
        vm.serializeAddress(root, "routerCore", router);
        if (ccipAdapter != address(0)) {
            vm.serializeAddress(root, "ccipAdapter", ccipAdapter);
        }

        // serialize chainId as string for convenience
        vm.serializeString(root, "chainId", _toString(chainId));

        // finalize to JSON string
        string memory json = vm.serializeString(root, "network", _networkName(chainId));

        // Write to file
        string memory file = string.concat(DEPLOY_DIR, "/", _toString(chainId), ".json");
        vm.writeJson(json, file);

        console2.log("Deployment written to:", file);
    }

    /* ------------------------------------------------------------ */
    /*                       HELPER: ENV KEYS                       */
    /* ------------------------------------------------------------ */

    /// @dev Try to read an address env var. If missing or invalid, returns address(0)
    function _maybeEnvAddress(string memory key) internal returns (address) {
        // try/catch cheatcode guard
        try this._readEnvAddress(key) returns (address a) {
            return a;
        } catch {
            return address(0);
        }
    }

    /// @dev Wrapper because try/catch cannot catch vm.envAddress directly in the same context
    function _readEnvAddress(string memory key) external returns (address) {
        return vm.envAddress(key);
    }

    /// @dev Read an array of addresses from ADAPTERS_JSON='["0x..","0x.."]'
    function _maybeEnvAddressArray(string memory key) internal returns (address[] memory out) {
        try this._readEnvJson(key) returns (string memory raw) {
            bytes memory file = bytes(raw);
            if (file.length == 0) return out;

            // Use stdJson via vm to parse array
            // e.g., we expect a top-level JSON array
            bytes memory arr = vm.parseJson(file, "");
            // Foundry returns ABI-encoded data for the parsed JSON; decode as address[]
            out = abi.decode(arr, (address[]));
            return out;
        } catch {
            return out; // empty
        }
    }

    /// @dev Wrapper to read a string env (JSON text)
    function _readEnvJson(string memory key) external returns (string memory) {
        return vm.envString(key);
    }

    /* ------------------------------------------------------------ */
    /*                      HELPER: FORMATTING                      */
    /* ------------------------------------------------------------ */

    function _toString(uint256 value) internal pure returns (string memory) {
        // minimal uint256 → string
        if (value == 0) return "0";
        uint256 temp = value;
        uint256 digits;
        while (temp != 0) {
            digits++;
            temp /= 10;
        }
        bytes memory buffer = new bytes(digits);
        while (value != 0) {
            digits -= 1;
            buffer[digits] = bytes1(uint8(48 + uint256(value % 10)));
            value /= 10;
        }
        return string(buffer);
    }

    function _networkName(uint256 chainId) internal pure returns (string memory) {
        // convenience names for common chains (extend as needed)
        if (chainId == 1) return "ethereum";
        if (chainId == 56) return "bsc";
        if (chainId == 137) return "polygon";
        if (chainId == 42161) return "arbitrum";
        if (chainId == 10) return "optimism";
        if (chainId == 8453) return "base";
        if (chainId == 43114) return "avalanche";
        return "unknown";
    }
}
