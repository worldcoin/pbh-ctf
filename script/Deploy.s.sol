// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {PBHKotH} from "src/PBHKingOfTheHill.sol";

contract Deploy is Script {
    PBHKotH public pbhKingOfTheHill;
    uint256 public constant GAME_START = 10152556;

    function run() public {
        uint256 deployer = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployer);
        pbhKingOfTheHill = new PBHKotH(GAME_START);
        vm.stopBroadcast();
    }
}
