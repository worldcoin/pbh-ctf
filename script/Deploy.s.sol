// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {PBHKotH} from "src/PBHKingOfTheHill.sol";

contract Deploy is Script {
    PBHKotH public pbhKingOfTheHill;

    function run() public {
        uint256 deployer = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployer);
        pbhKingOfTheHill = new PBHKotH();
        vm.stopBroadcast();
    }
}
