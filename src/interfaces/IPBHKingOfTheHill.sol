// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

interface IPBHKotH {
    function ctf(address addr) external;
    function gameEnd() external view returns (uint128);
    function latestBlock() external view returns (uint128);
    function leader() external view returns (address);
    function highScore() external view returns (uint256);
    function leaderboard(address addr) external view returns (uint256);
}
