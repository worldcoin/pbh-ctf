use alloy_primitives::Address;
use alloy_signer_local::PrivateKeySigner;
use pbh_ctf::{king_of_the_hill_multicall, world_id::WorldID, CTFTransactionBuilder, PBH_CTF_CONTRACT_TEST};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // The private key of the transaction signer
    let signer = std::env::var("PRIVATE_KEY")?.parse::<PrivateKeySigner>()?;

    // The semaphore secret
    let secret = std::env::var("SECRET")?;
    let world_id = WorldID::new(&secret)?;

    let player = Address::random();
    let calls = king_of_the_hill_multicall(player, PBH_CTF_CONTRACT_TEST);
    let pbh_nonce = 0;

    // The CTF Transaction Builder implements the builder pattern for creating a CTF transaction.
    // All `TransactionRequest` functions are accsessible through this builder
    let _tx = CTFTransactionBuilder::new()
        .with_pbh_multicall(&world_id, pbh_nonce, signer.address(), calls)
        .await?
        .build(signer)
        .await?;

    Ok(())
}
