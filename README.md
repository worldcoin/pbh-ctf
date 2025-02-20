# pbh-ctf

```

                    ██████  ██████  ██   ██      ██████ ████████ ███████ 
                    ██   ██ ██   ██ ██   ██     ██         ██    ██      
                    ██████  ██████  ███████     ██         ██    █████   
                    ██      ██   ██ ██   ██     ██         ██    ██      
                    ██      ██████  ██   ██      ██████    ██    ██      
    
                  
                                  __
                                 / \--..____
                                  \ \       \-----,,,..
                                   \ \       \         \--,,..
                                    \ \       \         \  ,'
                                     \ \       \         \ ``..
                                      \ \       \         \-''
                                       \ \       \__,,--'''
                                        \ \       \.
                                         \ \      ,/
                                          \ \__..-
                                           \ \
                                            \ \
                                             \ \   
                                              \ \
                                               \ \
                                                \ \
                                                 \ \
                                                  \ \
                                                   \ \
```


## Overview 
With the launch of Priority Blockspace for Humans (PBH) on World Chain Sepolia, a PBH CTF event will take place from `2025-02-28T05:00:00Z` to `2025-03-08T04:59:00Z` to discover edge cases and observe interesting/unexpected outcomes.

Priority Blockspace for Humans is a new transaction ordering policy on World Chain Sepolia that grants World ID holders top-of-block inclusion, reducing friction and making transactions fairer for real users.

This CTF event will feature a warm-up game followed by a "break things" track where participants will try to break specific invariants related to PBH. A bounty of (amount to be announced) will be paid out to the winner of the warm up game. Additionally bounties (amount to be announced) will be paid to participants that successfully break invariants specified below. 


## Getting a Testnet World ID
To submit PBH transactions, you will need a Testnet World ID.

In order to simulate realistic conditions and avoid participants creating multiple Testnet identities, we can leverage the World App as a sybil resistance mechanism when registering Testnet World IDs for the CTF event. Ensuring one Testnet identity per user will also allow us to learn how users will try to find loop-holes to get more PBH transactions.

Note that you simply need to download the app temporarily and do not need to verify. After provisioning the CTF semaphore secret key, you can delete the app if you wish. Follow this link to download the app:
https://worldcoin.org/world-app?download


Once you have the app downloaded, you can provision a new Testnet World ID by clicking this link and following the instructions on the page:
https://ctf-onboarding.stage-crypto.worldcoin.dev/front


**IMPORTANT:** Note that the semaphore key generated is visible to the registration service. This Testnet identity should not be used for anything other than the PBH CTF event. You can revisit the link above at anytime to see your semaphore key in the event that you misplace the value.



## Warmup Game: PBH King of the Hill
- **Start Time:** `2025-02-28T05:00:00Z`
- **End Time:** `2025-03-02T04:59:00Z`

### Details
The warm-up game is a simple "King of the Hill" game where participants race to call the `ctf()` function and increment a counter. Users will specify an address to be used as the key in the `leaderboard` mapping.

Each block, the first player to call the function will score a point. At the end of the time period, the player with the highest score will be sent the bounty (Amount to be announced) on World Chain Mainnet.

```solidity
contract PBHKotH {
    // --snip--
    /// @notice Function to attempt to capture the flag
    function ctf(address addr) public {
        require(block.timestamp < gameEnd, GameOver());

        // Ensure ctf hasnt been called yet this block
        require(block.timestamp > lastBlock, TooLate());
        lastBlock = uint128(block.timestamp);

        // Adjust the user's score
        uint256 score = leaderboard[addr];
        score += 1;
        leaderboard[addr] = score;

        if (score > highScore) {
            leader = addr;
            highScore = score;
        }
    }
}
```

## Break Things
- **Start Time:** `2025-02-28T05:00:00Z`
- **End Time:** `2025-03-08T04:59:00Z`



## PBH Testnet Configuration
- `pbhNonceLimit`: 100
    - TODO: explain this value
- `pbhGasLimit`: 10,500,000
    - TODO: explain this value
- `verifiedBlockspaceCapacity`: 70%
    - TODO: explain this value

## Important Links
- [World Chain Builder repo](https://github.com/worldcoin/world-chain/tree/main/world-chain-builder/crates/world)
- [PBH Specs](https://worldcoin.github.io/world-chain/)
- [Inclusion Proof RPC endpoint](TODO:)
- [World ID docs](https://docs.world.org/world-id/reference/contracts#usage)

## Testnet Contract Addresses
- [WorldID](https://worldscan.org/TODO:): `0xcoffee`
- [PBHEntryPoint](https://worldscan.org/TODO:): `0xcoffee`
- [PBHSignatureAggregator](https://worldscan.org/TODO:): `0xcoffee`
- [PBHKingOfTheHill](https://worldscan.org/TODO:): Address To Be Deployed
