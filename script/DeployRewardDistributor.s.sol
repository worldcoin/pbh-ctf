// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import {Script, console} from "forge-std/Script.sol";
import {RewardDistributor} from "src/RewardDistributor.sol";

contract Deploy is Script {
    RewardDistributor public rewardDistributor;
    address public constant CLAIMANT = 0xEd9d6ab46080aF6c75EF96B7293a4118F94Bc4e0; 
    address public constant USDC = 0x79A02482A880bCE3F13e09Da970dC34db4CD24d1;
    uint256 public constant REWARD = 3000e6;

    function run() public {
        uint256 deployer = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployer);
        rewardDistributor = new RewardDistributor(USDC, CLAIMANT, REWARD);
        vm.stopBroadcast();
    }
}
