# PBH CTF Starter Bot

## Installing
First, make sure Rust is installed.
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Once Rust is installed you can run the following command to install the starter bot.
```bash
git clone git@github.com:worldcoin/pbh-ctf.git
cd pbh-ctf/crates/pbh-ctf
cargo install --path .
```

## Configuration 

To configure the bot, edit the `pbh_ctf.toml` to include your `semaphore_secret`, RPC endpoint and the address that will be used to accumulate your score. This address can be separate from the account used to submit transactions. 
```toml
semaphore_secret = ""
provider_uri = ""
player_address = ""
```

## Running the Bot