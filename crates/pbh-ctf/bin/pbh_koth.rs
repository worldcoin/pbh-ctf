//! PBH CTF starter bot
pub mod config;

use std::{path::PathBuf, sync::Arc};

use alloy_network::Network;
use alloy_network::eip2718::Encodable2718;
use alloy_provider::{Provider, ProviderBuilder, WsConnect};
use alloy_signer_local::PrivateKeySigner;
use config::CTFConfig;
use eyre::eyre::{Result, eyre};
use futures::StreamExt;
use pbh_ctf::{
    CTFTransactionBuilder, PBH_CTF_CONTRACT, PBH_ENTRY_POINT,
    bindings::{IPBHEntryPoint::IPBHEntryPointInstance, IPBHKotH::IPBHKotHInstance},
    world_id::WorldID,
};
use reqwest::Url;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("bin/pbh_koth.toml");
    let config = CTFConfig::load(Some(config_path.as_path()))?;
    let private_key = std::env::var("PRIVATE_KEY")?;
    let signer = private_key.parse::<PrivateKeySigner>()?;
    let provider = Arc::new(
        ProviderBuilder::new()
            .on_ws(WsConnect::new(config.provider_uri.parse::<Url>()?))
            .await?,
    );

    // Initialize the WorldID
    let world_id = WorldID::new(&config.semaphore_secret)?;

    // Initialize the King of the Hill contract
    let pbh_koth = IPBHKotHInstance::new(PBH_CTF_CONTRACT, provider.clone());
    let game_start = pbh_koth.latestBlock().call().await?._0;
    let game_end = pbh_koth.gameEnd().call().await?._0;

    // Initialize the PBHEntrypoint contract and get the PBH nonce limit
    let pbh_entrypoint = IPBHEntryPointInstance::new(PBH_ENTRY_POINT, provider.clone());
    let pbh_nonce_limit = pbh_entrypoint.numPbhPerMonth().call().await?._0;

    // Subscribe to new blocks and prepare CTF transactions
    let mut block_stream = provider.subscribe_blocks().await?.into_stream();

    let player = signer.address();
    let mut pbh_nonce = get_pbh_nonce(&world_id, provider.clone(), pbh_nonce_limit).await?;

    let mut wallet_nonce = provider.get_transaction_count(signer.address()).await?;

    while let Some(header) = block_stream.next().await {
        if header.number > game_end.to() {
            println!("The game has ended, thanks for playing!");
            break;
        }

        if header.number < game_start.to() {
            println!("The game has not started yet, please wait...");
            continue;
        }

        // If the user has not hit the pbh limit send a PBH tx, otherwise send a standard tx
        let tx = if pbh_nonce < pbh_nonce_limit {
            tracing::info!("Preparing PBH CTF transaction");
            let calls = pbh_ctf::king_of_the_hill_multicall(player, PBH_CTF_CONTRACT);
            let tx = CTFTransactionBuilder::new()
                .to(PBH_ENTRY_POINT)
                .nonce(wallet_nonce)
                .from(signer.address())
                .with_pbh_multicall(&world_id, pbh_nonce, signer.address(), calls)
                .await?
                .build(signer.clone())
                .await?;

            // Optimistically bump the pbh nonce
            // @dev If the pbh transaction reverts, the PBH nonce will not be spent and can be used again
            pbh_nonce += 1;

            tx
        } else {
            tracing::info!("Preparing CTF transaction");
            let calldata = pbh_ctf::king_of_the_hill_calldata(player);
            CTFTransactionBuilder::new()
                .to(PBH_CTF_CONTRACT)
                .nonce(wallet_nonce)
                .from(signer.address())
                .input(calldata.into())
                .build(signer.clone())
                .await?
        };
        let pending_tx = provider.send_raw_transaction(&tx.encoded_2718()).await?;
        tracing::info!("Sent transaction: {:?}", pending_tx.tx_hash());
        wallet_nonce += 1;
    }

    Ok(())
}

async fn get_pbh_nonce<P: Provider<N>, N: Network>(
    world_id: &WorldID,
    provider: P,
    max_pbh_nonce: u16,
) -> Result<u16> {
    let start_nonce = 0;
    let pbh_entrypoint_instance = IPBHEntryPointInstance::new(PBH_ENTRY_POINT, provider);
    for i in start_nonce..=max_pbh_nonce {
        let nullifier_hash = world_id.pbh_ext_nullifier(i).2;
        let is_used = pbh_entrypoint_instance
            .nullifierHashes(nullifier_hash)
            .call()
            .await?
            ._0;
        if !is_used {
            return Ok(i);
        }
    }

    Err(eyre!("No available PBH nonce"))
}
