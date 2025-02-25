// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {PBHKotH} from "src/PBHKingOfTheHill.sol";

contract Deploy is Script {
    PBHKotH public pbhKingOfTheHill;
    uint128 public constant GAME_START = 1740700800; // 02/28/25 00:00:00

    function run() public {
        uint256 deployer = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployer);
        pbhKingOfTheHill = new PBHKotH(block.number);
        vm.stopBroadcast();
    }
}
