use alloy_signer_local::PrivateKeySigner;
use pbh_ctf::CTFTransactionBuilder;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // The private key of the transaction signer
    let signer = std::env::var("PRIVATE_KEY")?.parse::<PrivateKeySigner>()?;

    // The semaphore secret
    let secret = std::env::var("SECRET")?;

    // The CTF Transaction Builder implements the builder pattern for creating a CTF transaction.
    // All `TransactionRequest` functions are accsessible through this builder
    let tx = CTFTransactionBuilder::new().build(signer).await?;

    Ok(())
}
