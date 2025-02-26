// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {PBHKotH} from "../src/PBHKingOfTheHill.sol";

contract PBHKotHTest is Test {
    PBHKotH public pbhKingOfTheHill;

    function setUp() public {
        pbhKingOfTheHill = new PBHKotH(block.number);
    }

    function test_Ctf_RevertIf_GameOver() public {
        vm.roll(block.number + 129600);
        vm.expectRevert(PBHKotH.GameOver.selector);
        pbhKingOfTheHill.ctf(address(this));
    }

    function testCtf_RevertIf_TooLate() public {
        vm.roll(block.number + 1);
        pbhKingOfTheHill.ctf(address(this));
        vm.prank(address(0xc0ffee));
        vm.expectRevert(PBHKotH.TooLate.selector);
        pbhKingOfTheHill.ctf(address(0xc0ffee));
    }

    function testCtf() public {
        vm.roll(block.number + 1);
        pbhKingOfTheHill.ctf(address(this));
        assertEq(pbhKingOfTheHill.leader(), address(this));
        assertEq(pbhKingOfTheHill.highScore(), 1);
        assertEq(pbhKingOfTheHill.leaderboard(address(this)), 1);
        vm.startPrank(address(0xc0ffee));
        vm.roll(block.number + 2);
        pbhKingOfTheHill.ctf(address(0xc0ffee));
        vm.roll(block.number + 3);
        pbhKingOfTheHill.ctf(address(0xc0ffee));
        assertEq(pbhKingOfTheHill.leader(), address(0xc0ffee));
        assertEq(pbhKingOfTheHill.highScore(), 2);
        assertEq(pbhKingOfTheHill.leaderboard(address(0xc0ffee)), 2);
    }
}
