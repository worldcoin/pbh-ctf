use alloy_primitives::Address;
use alloy_signer_local::PrivateKeySigner;
use pbh_ctf::CTFTransactionBuilder;

use pbh_ctf::PBH_CTF_CONTRACT;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // The private key of the transaction signer
    let signer = std::env::var("PRIVATE_KEY")?.parse::<PrivateKeySigner>()?;

    let player = Address::random();
    let calldata = pbh_ctf::king_of_the_hill_calldata(player);

    let _tx = CTFTransactionBuilder::new()
        .to(PBH_CTF_CONTRACT)
        .input(calldata.into())
        .build(signer)
        .await?;

    Ok(())
}
