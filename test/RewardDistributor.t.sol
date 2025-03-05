// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {Test, console} from "forge-std/Test.sol";

import {RewardDistributor} from "../src/RewardDistributor.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract UniswapV3Callback {
    function uniswapV3SwapCallback(int256 amount0Delta, int256 amount1Delta, bytes calldata data) external {
        assembly {
            let freeMemoryPointer := mload(0x40)
            let token := calldataload(data.offset)
            mstore(freeMemoryPointer, 0xa9059cbb00000000000000000000000000000000000000000000000000000000)
            mstore(add(freeMemoryPointer, 4), and(caller(), 0xffffffffffffffffffffffffffffffffffffffff))
            switch slt(amount0Delta, 0)
            case 0 { mstore(add(freeMemoryPointer, 36), amount0Delta) }
            default { mstore(add(freeMemoryPointer, 36), amount1Delta) }

            if iszero(
                and(
                    or(and(eq(mload(0), 1), gt(returndatasize(), 31)), iszero(returndatasize())),
                    call(gas(), token, 0, freeMemoryPointer, 68, 0, 32)
                )
            ) {
                // Revert if the call failed.
                revert(0, 0)
            }
        }
    }
}

contract RewardDistributorTest is UniswapV3Callback, Test {
    address public constant USDC = 0x79A02482A880bCE3F13e09Da970dC34db4CD24d1;
    address public constant WETH = 0x4200000000000000000000000000000000000006;
    address public constant CLAIMANT = address(0xc0ffee);
    address public constant AUTHORITY = address(0xb00b);
    address public constant UNISWAP_V3_FACTORY = 0x7a5028BDa40e7B173C278C5342087826455ea25a;
    address public immutable USDC_WETH = IUniswapV3Factory(UNISWAP_V3_FACTORY).getPool(USDC, WETH, 500);
    uint256 public constant REWARD = 3000e6;
    uint160 internal constant MIN_SQRT_RATIO = 4295128739;

    RewardDistributor public rewardDistributor;

    function setUp() public {
        vm.deal(address(this), type(uint128).max);
        IWeth(WETH).deposit{value: type(uint128).max}();
        IUniswapV3Pool(USDC_WETH).swap(address(this), true, 2e18, MIN_SQRT_RATIO + 1, abi.encode(WETH));
        rewardDistributor = new RewardDistributor(USDC, AUTHORITY, CLAIMANT, REWARD);
        IERC20(USDC).transfer(address(rewardDistributor), REWARD);
    }

    // Claim
    function testFuzz_Claim_RevertIf_Unauthorized(address claimant, address receiver) public {
        vm.assume(claimant != CLAIMANT);
        vm.prank(claimant);
        vm.expectRevert(RewardDistributor.Unauthorized.selector);
        rewardDistributor.claim(receiver);
    }

    function testClaim_RevertIf_Locked() public {
        vm.prank(CLAIMANT);
        vm.expectRevert(RewardDistributor.Locked.selector);
        rewardDistributor.claim(CLAIMANT);
    }

    function testClaim() public {
        // Unlock the reward
        vm.prank(AUTHORITY);
        rewardDistributor.unlock();
        // Claim the reward
        vm.prank(CLAIMANT);
        rewardDistributor.claim(CLAIMANT);
        assertEq(IERC20(USDC).balanceOf(CLAIMANT), REWARD);
    }

    // Unlock
    function testFuzz_Unlock_RevertIf_Unauthorized(address authority) public {
        vm.assume(authority != AUTHORITY);
        vm.prank(authority);
        vm.expectRevert(RewardDistributor.Unauthorized.selector);
        rewardDistributor.unlock();
    }

    function test_Unlock() public {
        vm.prank(AUTHORITY);
        rewardDistributor.unlock();
        assert(rewardDistributor.unlocked());
    }

    // Skim
    function test_Skim() public {
        vm.prank(AUTHORITY);
        rewardDistributor.skim(AUTHORITY, REWARD);
        assertEq(IERC20(USDC).balanceOf(AUTHORITY), REWARD);
    }

    function testFuzz_Skim_RevertIf_Unauthorized(address authority) public {
        vm.assume(authority != AUTHORITY);
        vm.prank(authority);
        vm.expectRevert(RewardDistributor.Unauthorized.selector);
        rewardDistributor.skim(authority, REWARD);
    }
}

interface IUniswapV3Factory {
    function getPool(address tokenA, address tokenB, uint24 fee) external view returns (address pool);
}

interface IUniswapV3Pool {
    function swap(
        address recipient,
        bool zeroForOne,
        int256 amountSpecified,
        uint160 sqrtPriceLimitX96,
        bytes calldata data
    ) external returns (int256 amount0, int256 amount1);

    function token0() external view returns (address);
    function token1() external view returns (address);
}

interface IWeth {
    function deposit() external payable;
}
