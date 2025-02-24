use alloy_sol_types::sol;

sol! {
    #[sol(rpc)]
    interface IPBHKotH {
        function ctf(address receiver) external;
        function gameEnd() external view returns (uint128);
        function latestBlock() external view returns (uint128);
        function leader() external view returns (address);
        function highScore() external view returns (uint256);
        function leaderboard(address addr) external view returns (uint256);
    }
}
