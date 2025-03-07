// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {Ownable2Step} from "@openzeppelin/contracts/access/Ownable2Step.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

/// @title Reward Distributor
/// @notice Distributes PBH King of the Hill reward to the winning participant.
contract RewardDistributor is Ownable2Step {
    using SafeERC20 for IERC20;

    /// @notice Address of the USDC.e contract.
    IERC20 public immutable usdc;

    /// @notice Address of the PBH King of the Hill winner.
    address public immutable claimant;

    /// @notice Amount of USDC to be distributed to the `claimant`.
    uint256 public immutable reward;

    modifier onlyClaimant() {
        if (msg.sender != claimant) {
            revert Unauthorized();
        }

        _;
    }

    /// @notice Event emitted when the reward is claimed.
    /// @param claimant Address of the claimant.
    /// @param reward Amount of USDC claimed.
    event RewardClaimed(address indexed claimant, uint256 indexed reward);

    /// @notice Thrown when the caller is not authorized to call the given function.
    error Unauthorized();

    /// @notice thrown when a constructor parameter is zero.
    error ZeroValue();

    constructor(address _usdc, address _claimant, uint256 _reward) Ownable(msg.sender) {
        if (_usdc == address(0) || _claimant == address(0) || _reward == 0) {
            revert ZeroValue();
        }

        usdc = IERC20(_usdc);
        claimant = _claimant;
        reward = _reward;
    }

    /// @notice Function to claim the reward.
    /// @param receiver Address where the reward will be transferred.
    function claim(address receiver) external onlyClaimant {
        usdc.safeTransfer(receiver, reward);
        emit RewardClaimed(claimant, reward);
    }

    /// @notice Function to withdraw any superfelous funds from the contract.
    /// @param token Address of the token to be withdrawn.
    /// @param receiver Address where the funds will be transferred.
    /// @param value Amount of USDC to be transferred.
    function withdrawFunds(address token, address receiver, uint256 value) external onlyOwner {
        IERC20(token).safeTransfer(receiver, value);
    }
}
