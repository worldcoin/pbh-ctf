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

**TL;DR** 

  ⚡️ PBH King of the Hill - 🏎️ 3k USDC Winner takes all 🏎️ 

  ⚡️ PBH Security Track - 🔒 Up to 40k USDC in available bounties 🔒

# Table of Contents

- [Overview](#overview)
- [Docs](#docs)
- [Getting a Testnet World ID](#getting-a-testnet-world-id)
- [Warmup Game: PBH King of the Hill](#warmup-game-pbh-king-of-the-hill)
  - [Details](#details)
- [Break PBH Track](#break-pbh-track)
  - [Details](#details-1)
  - [Invariants](#invariants)
- [PBH Testnet Configuration](#pbh-testnet-configuration)
- [Important Links](#important-links)
- [Testnet Contract Addresses](#testnet-contract-addresses)

<br>

## Overview 
With the launch of Priority Blockspace for Humans (PBH) on World Chain Sepolia, a PBH CTF event will take place from `2025-02-28T05:00:00Z` to `2025-03-08T04:59:00Z` to discover edge cases and observe interesting/unexpected outcomes.

Priority Blockspace for Humans is a new transaction ordering policy on World Chain Sepolia that grants World ID holders top-of-block inclusion, reducing friction and making transactions fairer for real users.

This CTF event will feature a warm-up game followed by a "Break PBH" track where participants will try to break specific invariants related to PBH.


## Docs
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
https://pbh-ctf.worldcoin.dev/front/#/register

**IMPORTANT:** Note that the semaphore key generated is not secure and is visible to the registration service. This Testnet identity should not be used for anything other than the PBH CTF event. In the event that you misplace your Testnet World ID, you can revisit the link above and follow the steps again at anytime to see your ID.


<br>

## Warmup Game: PBH King of the Hill
- **Kick off:** `2025-02-28T05:00:00Z`
- **Game Start Time:** `2025-03-01T18:30:00Z` (block `10152556`)
- **Game End Time:** `2025-03-02T04:59:00Z`

### Details
The warm-up game is a simple "King of the Hill" game where participants race to call the `ctf()` function and increment a counter. Users will specify an address to be used as the key in the `leaderboard` mapping.

Each block, the first player to call the function will score a point. At the end of the time period, the player with the highest score will be sent the bounty on World Chain Mainnet. PBH will allow users to be included in the block with priority over non-PBH transactions. Note that if there are multiple PBH transactions in the block, this subset of transactions is sorted by priority fee.

The event will start at `2025-02-28T05:00:00Z` where builders can start building their bot and ask any questions in the [PBH CTF telegram group](https://t.me/pbhctf). The King of the Hill contract will unlock at `2025-03-01T18:30:00Z` (block `10152556`), allowing participants to start submitting transactions and accumulating their score. The game will end at `2025-03-02T04:59:00Z` and the player with the highest score will win.  
A bounty of $3k USDC will be paid out on World Chain Mainnet to the winner of the warm-up game.

Check out the [starter bot](./crates/pbh-ctf/bin/pbh_koth.rs) and create your own implementation.

```solidity
contract PBHKotH {
    // --snip--

    /// @notice Function to attempt to capture the flag
    /// @dev This can only be called once per block
    function ctf(address receiver) public {
        // Ensure game is still active
        require(block.number < gameEnd, GameOver());

        // Ensure ctf hasnt been called yet this block
        require(block.number > latestBlock, TooLate());
        latestBlock = uint128(block.number);

        // Adjust the user's score
        uint256 score = leaderboard[receiver];

        score += 1;
        leaderboard[receiver] = score;

        // Adjust high score/leader if score > highScore
        if (score > highScore) {
            leader = receiver;
            highScore = score;
        }

        emit Ctf(receiver, score);
    }
}
```

### Starter bot installation

To install the PBH CTF binary run the following command:

```shell
cargo install --path crates/pbh-ctf
```

Then to run the binary run:
```shell
pbh_koth
```

Remember to set the proper fields in the `pbh_koth.toml` and set the `PRIVATE_KEY` environment variable with your private key.

<br>

## Break PBH Track
- **Start Time:** `2025-02-28T05:00:00Z`
- **End Time:** `2025-03-08T04:59:00Z`

### Details
This portion of the CTF event is focused on breaking PBH invariants. 
There are four invariants that should always hold true when the World Chain Builder is producing blocks. 
It is important to note that if the builder block is not selected, these conditions are not enforced. Participants can verify if the World Chain Builder built a specific block by querying the [builtBlock mapping in the WorldChainBlockRegistry contract](https://worldchain-sepolia.explorer.alchemy.com/address/0x46CD6926427A2190310eBA2BF713F0EF51dFB59F?tab=read_write_contract#0xc21612a7).

The **total bounty for each invariant is capped at $10k**, regardless of the number of valid submissions. Participants that submit a valid proof of concept and write-up demonstrating how to break an invariant will be eligible for a portion of the bounty.

Bounty distribution will follow the rules below:
- Each submission will be evaluated for validity and severity of impact. Each valid submission will be evaluated for a portion of the bounty. The **total bounty for each invariant capped at $10k**. 
- If multiple participants submit the **same finding**, only the first valid submission will be evaluated for a bounty.
- If multiple participants break an invariant in **distinctly different ways**, each submission will be evaluated for a separate bounty.
- If there are `n` distinct valid findings for an invariant, the **total bounty for that invariant remains capped at $10k**, and each submission will be evaluated for a bounty.
- Bounties may not be distributed to participants if it is determined that their submission or the payment of a bounty would violate applicable laws.


Participants can submit findings via this link: (Link to be added)

### Invariants

For any block built by the World Chain Builder, the following invariants must hold. It is important to note that if the builder block is not selected, these conditions are not enforced. 

- **PBH Transaction Limits**: Users cannot exceed `numPbhPerMonth` PBH transactions per month, for a given World ID.

- **PBH Gas per UserOp/Tx**: No single PBH UserOp or PBH transaction can exceed `pbhGasLimit`.

- **PBH Block Limit**: The total PBH gas in a block must not exceed `pbhBlockCapacity`.

- **PBH Ordering Rules**: All PBH transactions must be ordered before non-PBH transactions in a block (other than sequencer transactions (eg. `setL1BlockValuesEcotone`, Deposit transactions, etc.).

<br>

## PBH Testnet Configuration
PBH on World Chain Sepolia will be configured with the following parameters.

- `pbhNonceLimit`: 65536
    - The amount of PBH transactions a user can submit per month, per World ID.
- `pbhGasLimit`: 15,000,000
    - The maximum gas that a single PBH tx / UserOp can spend.
- `verifiedBlockspaceCapacity`: 70%
    - The maximum amount of PBH gas in a block. This is calculated as a percentage of the block limit.

<br>

## Important Links
- [World Chain Builder](https://github.com/worldcoin/world-chain)
- [PBH Specs](https://worldcoin.github.io/world-chain/pbh/architecture.html)
- [World ID docs](https://docs.world.org/world-id/reference/contracts#usage)
- [World Chain Sepolia Faucet](https://www.alchemy.com/faucets/world-chain-sepolia)
- [PBH CTF telegram group](https://t.me/pbhctf)

<br>

## Testnet Contract Addresses
- [WorldID](https://worldchain-sepolia.explorer.alchemy.com/address/0xE177F37AF0A862A02edFEa4F59C02668E9d0aAA4): `0xE177F37AF0A862A02edFEa4F59C02668E9d0aAA4`
- [PBHEntryPoint](https://worldchain-sepolia.explorer.alchemy.com/address/0x6e37bAB9d23bd8Bdb42b773C58ae43C6De43A590): `0x6e37bAB9d23bd8Bdb42b773C58ae43C6De43A590`
- [PBHSignatureAggregator](https://worldchain-sepolia.explorer.alchemy.com/address/0xf07d3efadD82A1F0b4C5Cc3476806d9a170147Ba): `0xf07d3efadD82A1F0b4C5Cc3476806d9a170147Ba`
- [PBHKingOfTheHill](https://worldchain-sepolia.explorer.alchemy.com/address/0x0432c59e03969Ca5B747023E43B6fa2aEe83AEd5?tab=txs): `0x0432c59e03969Ca5B747023E43B6fa2aEe83AEd5`
