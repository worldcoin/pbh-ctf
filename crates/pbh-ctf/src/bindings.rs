use alloy_sol_types::sol;

sol! {
    #[sol(rpc)]
    interface IPBHKotH {
        function ctf(address receiver) external;
        function gameEnd() external view returns (uint256);
        function latestBlock() external view returns (uint256);
        function leader() external view returns (address);
        function highScore() external view returns (uint256);
        function leaderboard(address addr) external view returns (uint256);
    }

    #[sol(rpc)]
    interface IPBHEntryPoint {
        function numPbhPerMonth() external view returns (uint16);
        function nullifierHashes(uint256) external view returns (bool);
    }
}
