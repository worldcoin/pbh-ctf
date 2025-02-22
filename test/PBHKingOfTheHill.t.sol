// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {PBHKotH} from "../src/PBHKingOfTheHill.sol";

contract PBHKotHTest is Test {
    PBHKotH public pbhKingOfTheHill;

    function setUp() public {
        pbhKingOfTheHill = new PBHKotH();
    }

    function test_Ctf_RevertIf_GameOver() public {
        vm.warp(block.timestamp + 3 days);
        vm.expectRevert(PBHKotH.GameOver.selector);
        pbhKingOfTheHill.ctf();
    }

    function testCtf_RevertIf_tooLate() public {
        vm.warp(block.timestamp + 1);
        pbhKingOfTheHill.ctf();
        vm.prank(address(0xc0fee));
        vm.expectRevert(PBHKotH.TooLate.selector);
        pbhKingOfTheHill.ctf();
    }

    function testCtf() public {
        vm.warp(block.timestamp + 1);
        pbhKingOfTheHill.ctf();
        assertEq(pbhKingOfTheHill.leader(), address(this));
        assertEq(pbhKingOfTheHill.highScore(), 1);
        assertEq(pbhKingOfTheHill.leaderboard(address(this)), 1);
        vm.startPrank(address(0xc0ffee));
        vm.warp(block.timestamp + 2);
        pbhKingOfTheHill.ctf();
        vm.warp(block.timestamp + 3);
        pbhKingOfTheHill.ctf();
        assertEq(pbhKingOfTheHill.leader(), address(0xc0ffee));
        assertEq(pbhKingOfTheHill.highScore(), 2);
        assertEq(pbhKingOfTheHill.leaderboard(address(0xc0ffee)), 2);
    }
}
