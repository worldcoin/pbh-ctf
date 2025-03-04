// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

/// @title Reward Distributor
/// @notice Distributes PBH King of the Hill reward to the winning participant.
contract RewardDistributor {
    using SafeERC20 for IERC20;

    /// @notice Address of the USDC.e contract.
    IERC20 public immutable usdc;

    /// @notice Address of the PBH King of the Hill winner.
    address public immutable claimant;

    /// @notice Party authorized to release the reward to the `claimant`.
    address public immutable authority;

    /// @notice Amount of USDC to be distributed to the `claimant`.
    uint256 public immutable reward;

    /// @notice Bool indicating whether the reward is able to be claimed.
    bool public unlocked;

    modifier onlyAuthority() {
        if (msg.sender != authority) {
            revert Unauthorized();
        }
        _;
    }

    modifier onlyClaimant() {
        if (msg.sender != claimant) {
            revert Unauthorized();
        }

        if (!unlocked) {
            revert Locked();
        }
        _;
    }

    /// @notice Event emitted when the reward is claimed.
    /// @param claimant Address of the claimant.
    /// @param reward Amount of USDC claimed.
    event RewardClaimed(address indexed claimant, uint256 indexed reward);

    /// @notice Event emitted when the reward is unlocked.
    event RewardUnlocked();

    /// @notice Thrown when the caller is not authorized to call the given function.
    error Unauthorized();

    /// @notice thrown when a constructor parameter is zero.
    error ZeroValue();

    /// @notice Thrown when the reward is not unlocked.
    error Locked();

    constructor(address _usdc, address _authority, address _claimant, uint256 _reward) {
        if (_usdc == address(0) || _authority == address(0) || _claimant == address(0) || _reward == 0) {
            revert ZeroValue();
        }

        usdc = IERC20(_usdc);
        authority = _authority;
        claimant = _claimant;
        reward = _reward;
    }

    /// @notice Function to unlock the reward for the `claimant`.
    function unlock() external onlyAuthority {
        unlocked = true;
        emit RewardUnlocked();
    }

    /// @notice Function to claim the reward.
    /// @param receiver Address where the reward will be transferred.
    function claim(address receiver) external onlyClaimant {
        unlocked = false;
        usdc.safeTransfer(receiver, reward);
        emit RewardClaimed(claimant, reward);
    }

    /// @notice Function to skim any superfelous funds from the contract.
    /// @param receiver Address where the funds will be transferred.
    /// @param value Amount of USDC to be transferred.
    function skim(address receiver, uint256 value) external onlyAuthority {
        usdc.safeTransfer(receiver, value);
    }
}
