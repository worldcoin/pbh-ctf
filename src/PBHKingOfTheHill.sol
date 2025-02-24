// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {IPBHKotH} from "./interfaces/IPBHKingOfTheHill.sol";

/// @title PBH King of the Hill (PBHKotH)
/// @notice A competitive game where participants try to be the first to increment a counter within a block.
/// @dev The game lasts for 3 days, and the player with the highest score at the end wins.
contract PBHKotH is IPBHKotH {
    /// @notice The Address of the PBHEntryPoint contract.
    address public immutable entryPoint = 0xCDfDF72065493bDDb2131478c89C1D5482BD1dF6;

    /// @notice Timestamp marking the end of the game.
    uint128 public immutable gameEnd = uint128(block.timestamp) + 3 days;

    /// @notice Timestamp marking the latest block a ctf call was made.
    uint128 public immutable gameStart;

    /// @notice Latest block a ctf call was made. Marks the start of the CTF game.
    uint128 public latestBlock;

    /// @notice Address of the current leader.
    address public leader;

    /// @notice The current high score.
    uint256 public highScore;

    /// @notice Mapping of player addresses to their scores.
    mapping(address addr => uint256 score) public leaderboard;

    /// @notice Event emitted when a player successfully captures the flag.
    event Ctf(address indexed addr, uint256 score);

    /// @notice Thrown when trying to call ctf after the game has ended.
    error GameOver();

    /// @notice Thrown when ctf is called more than once per block.
    error TooLate();

    /// @param _gameStart Timestamp of the start of the CTF game.
    constructor(uint128 _gameStart) {
        latestBlock = _gameStart;
    }

    /// @notice Function to attempt to capture the flag
    /// @dev This can only be called once per block
    function ctf(address receiver) public {
        // Ensure game is still active
        require(block.timestamp < gameEnd, GameOver());

        // Ensure ctf hasnt been called yet this block
        require(block.timestamp > latestBlock, TooLate());
        latestBlock = uint128(block.timestamp);

        // Adjust the user's score
        uint256 score = leaderboard[msg.sender];

        // PBH transactions are weighted 400:1
        score = msg.sender == entryPoint ? score + 400 : score + 1;
        leaderboard[receiver] = score;

        // Adjust high score/leader if score > highScore
        if (score > highScore) {
            leader = receiver;
            highScore = score;
        }

        emit Ctf(receiver, score);
    }
}
