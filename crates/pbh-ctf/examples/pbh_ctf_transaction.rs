use alloy_primitives::Address;
use alloy_signer_local::PrivateKeySigner;
use pbh_ctf::{CTFTransactionBuilder, king_of_the_hill_multicall, world_id::WorldID};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // The private key of the transaction signer
    let signer = std::env::var("PRIVATE_KEY")?.parse::<PrivateKeySigner>()?;

    // The semaphore secret
    let secret = std::env::var("SECRET")?;
    let world_id = WorldID::new(&secret)?;

    let player = Address::random();
    let calls = king_of_the_hill_multicall(player);
    let pbh_nonce = 0;

    // The CTF Transaction Builder implements the builder pattern for creating a CTF transaction.
    // All `TransactionRequest` functions are accsessible through this builder
    let tx = CTFTransactionBuilder::new()
        .with_pbh_multicall(&world_id, pbh_nonce, calls)
        .await?
        .build(signer)
        .await?;

    Ok(())
}
