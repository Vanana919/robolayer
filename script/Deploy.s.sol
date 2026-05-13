// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import { Script, console2 } from "forge-std/Script.sol";
import { Robodyne } from "../contracts/Robodyne.sol";

/// @notice Foundry deploy script.
/// @dev    Usage:
///         forge script script/Deploy.s.sol --rpc-url $SEPOLIA_RPC_URL --broadcast --verify
contract Deploy is Script {
    function run() external returns (Robodyne robo) {
        uint256 pk = vm.envUint("DEPLOYER_PRIVATE_KEY");
        address treasury = vm.envAddress("TREASURY_ADDRESS");
        uint256 minStake = vm.envOr("MIN_STAKE_WEI", uint256(1 ether));
        uint256 slashBps = vm.envOr("SLASH_SEVERITY_BPS", uint256(500));

        vm.startBroadcast(pk);
        robo = new Robodyne(minStake, slashBps, treasury);
        vm.stopBroadcast();

        console2.log("Robodyne deployed at:", address(robo));
        console2.log("minStake:", minStake);
        console2.log("slashSeverityBps:", slashBps);
        console2.log("treasury:", treasury);
    }
}
