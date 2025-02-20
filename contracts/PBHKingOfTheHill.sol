// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/// @title PBH King of the Hill (PBHKotH)
/// @notice A competitive game where participants try to be the first to increment a counter within a block.
/// @dev The game lasts for 3 days, and the player with the highest score at the end wins.
contract PBHKotH {
    uint128 public gameEnd = uint128(block.timestamp) + 3 days;
    uint128 public lastBlock = uint128(block.timestamp);
    address public leader;
    uint256 public highScore;

    mapping(address addr => uint256 score) public leaderboard;

    error GameOver();
    error TooLate();

    /// @notice Function to attempt to capture the flag
    /// @dev This can only be called once per block
    function ctf(address addr) public {
        require(block.timestamp < gameEnd, GameOver());

        // Ensure ctf hasnt been called yet this block
        require(block.timestamp > lastBlock, TooLate());
        lastBlock = uint128(block.timestamp);

        // Adjust the user's score
        uint256 score = leaderboard[addr];
        score += 1;
        leaderboard[addr] = score;

        // Adjust high score/leader if score > highScore
        if (score > highScore) {
            leader = addr;
            highScore = score;
        }
    }
}
