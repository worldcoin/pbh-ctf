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

<br>

## Overview 
With the launch of Priority Blockspace for Humans (PBH) on World Chain Sepolia, a PBH CTF event will take place from `2025-02-28T05:00:00Z` to `2025-03-08T04:59:00Z` to discover edge cases and observe interesting/unexpected outcomes.

Priority Blockspace for Humans is a new transaction ordering policy on World Chain Sepolia that grants World ID holders top-of-block inclusion, reducing friction and making transactions fairer for real users.

This CTF event will feature a warm-up game followed by a "Break PBH" track where participants will try to break specific invariants related to PBH. A bounty of (amount to be announced) will be paid out to the winner of the warm-up game. Additionally, bounties (amounts to be announced) will be paid to participants that successfully break invariants specified below. 

To get familiar with PBH and the World Chain Builder, check out these links:
- [PBH Docs](https://worldcoin.github.io/world-chain/pbh/architecture.html)
- [World Chain Builder](https://github.com/worldcoin/world-chain/tree/main/world-chain-builder/crates/world)
    - [Transaction Validation](https://github.com/worldcoin/world-chain/blob/main/world-chain-builder/crates/world/pool/src/validator.rs#L180)
    - [Payload Builder](https://github.com/worldcoin/world-chain/blob/main/world-chain-builder/crates/world/payload/src/builder.rs#L208)

<br>


## Getting a Testnet World ID
To submit PBH transactions, you will need a Testnet World ID.

To simulate realistic conditions and prevent CTF participants from creating multiple Testnet identities, we can use the World App as a sybil resistance mechanism during Testnet World ID registration for the event. Ensuring one Testnet identity per user will also allow us to learn more about how users will try to find loop-holes to get more PBH transactions.

Note that you simply need to download the app temporarily and do not need to verify. After provisioning the CTF World ID, you can delete the app if you would like. Follow this link to download the app:
https://worldcoin.org/world-app?download


Once you have the app downloaded, you can provision a new Testnet World ID by clicking this link and following the instructions on the page:
https://ctf-onboarding.stage-crypto.worldcoin.dev/front


**IMPORTANT:** Note that the semaphore key generated is not secure and is visible to the registration service. This Testnet identity should not be used for anything other than the PBH CTF event. In the event that you misplace your Testnet World ID, you can revisit the link above and follow the steps again at anytime to see your ID.


<br>


## Warmup Game: PBH King of the Hill
- **Start Time:** `2025-02-28T05:00:00Z`
- **End Time:** `2025-03-02T04:59:00Z`

### Details
The warm-up game is a simple "King of the Hill" game where participants race to call the `ctf()` function and increment a counter. Users will specify an address to be used as the key in the `leaderboard` mapping.

Each block, the first player to call the function will score a point. At the end of the time period, the player with the highest score will be sent the bounty (Amount to be announced) on World Chain Mainnet. PBH will allow users to be included in the block with priority over non-PBH transactions. Note that if there are multiple PBH transactions in the block, this subset of transactions is sorted by priority fee.

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

<br>

## Break PBH Track
- **Start Time:** `2025-02-28T05:00:00Z`
- **End Time:** `2025-03-08T04:59:00Z`

### Details
This portion of the CTF event is focused on breaking PBH invariants. There are four invariants that should always hold true when the World Chain Builder is producing blocks. 

Any participant that submits a valid proof of concept and write-up showing that they are able break an invariant will be paid a bounty (up to a specific amount, amounts to be announced). 

There will be a bounty to break each invariant (amounts to be announced). Participants that submit a valid proof of concept and write-up showing how to break an invariant will be eligible for a portion (or all) of the bounty.

Each submission will be evaluated for validity and severity of impact. If two participants submit the same finding, only the first submission will be paid the bounty. If two participants break an invariant in two distinctly different ways, both will be paid a separate bounty.

Participants can submit findings via this link: (Link to be added)

### Invariants

- **PBH Transaction Limits**: Users cannot exceed `numPbhPerMonth` PBH transactions per month, for a given World ID.

- **PBH Gas per UserOp/Tx**: No single PBH UserOp or PBH transaction can exceed `pbhGasLimit`.

- **PBH Block Limit**: The total PBH gas in a block must not exceed `pbhBlockCapacity`.

- **PBH Ordering Rules**: All PBH transactions must be ordered before non-PBH transactions in a block.


<br>

## PBH Testnet Configuration
PBH on World Chain Sepolia will be configured with the following parameters.

- `pbhNonceLimit`: 100
    - The amount of PBH transactions a user can submit per month, per World ID.
- `pbhGasLimit`: 10,500,000
    - The maximum gas that a single PBH tx / UserOp can spend.
- `verifiedBlockspaceCapacity`: 70%
    - The maximum amount of PBH gas in a block. This is calculated as a percentage of the block limit.

<br>

## Important Links
- [World Chain Builder](https://github.com/worldcoin/world-chain)
- [PBH Specs](https://worldcoin.github.io/world-chain/pbh/architecture.html)
- [World ID docs](https://docs.world.org/world-id/reference/contracts#usage)
- [World Chain Sepolia Faucet](https://www.alchemy.com/faucets/world-chain-sepolia)

<br>

## Testnet Contract Addresses
- [WorldID](https://worldchain-sepolia.explorer.alchemy.com/address/0xE177F37AF0A862A02edFEa4F59C02668E9d0aAA4): `0xE177F37AF0A862A02edFEa4F59C02668E9d0aAA4`
- [PBHEntryPoint](https://worldchain-sepolia.explorer.alchemy.com/address/0xCDfDF72065493bDDb2131478c89C1D5482BD1dF6): `0xCDfDF72065493bDDb2131478c89C1D5482BD1dF6`
- [PBHSignatureAggregator](https://worldchain-sepolia.explorer.alchemy.com/address/0xED5dc9CDB270818dCec0784bBdc8094082f0eBcB): `0xED5dc9CDB270818dCec0784bBdc8094082f0eBcB`
- [PBHKingOfTheHill](https://worldscan.org/TODO:): Address To Be Deployed
