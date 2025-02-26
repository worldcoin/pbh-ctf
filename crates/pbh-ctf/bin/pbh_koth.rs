//! PBH CTF starter bot
pub mod config;

use std::{path::PathBuf, pin::Pin, sync::Arc};

use alloy_eips::eip2718::Encodable2718;
use alloy_primitives::Bytes;
use alloy_provider::{Provider, ProviderBuilder, WsConnect};
use alloy_signer_local::PrivateKeySigner;
use async_stream::stream;
use config::CTFConfig;
use eyre::eyre::Result;
use futures::{Stream, StreamExt};
use pbh_ctf::derive_identity;
use pbh_ctf::{
    Identity, PBH_CTF_CONTRACT, PBH_ENTRY_POINT,
    bindings::{IPBHEntryPoint::IPBHEntryPointInstance, IPBHKotH::IPBHKotHInstance},
    ctf_transaction_builder, pbh_ctf_transaction_builder,
};
use reqwest::Url;

use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("pbh_koth.toml");
    let config = CTFConfig::load(Some(config_path.as_path()))?;
    let identity = derive_identity(&config.secret)?;

    let provider = Arc::new(
        ProviderBuilder::new()
            .on_ws(WsConnect::new(config.provider.parse::<Url>()?))
            .await?,
    );

    let mut tx_stream =
        subscribe_and_prepare(provider.clone(), identity, config.private_key).await?;

    let (tx, rx) = tokio::sync::mpsc::channel(100);
    let tx_manager = TxManager {
        receiver: rx,
        provider,
    };

    let tx_manager_handle = tokio::spawn(async move { tx_manager.run().await });

    let stream_fut = async {
        while let Some(transaction) = tx_stream.next().await {
            let transaction = transaction?;
            if let Some(transaction) = transaction {
                tx.send(transaction).await?;
            }
        }

        Ok::<(), eyre::Report>(())
    };

    tokio::select! {
        _ = stream_fut => {
            error!("Stream ended unexpectedly");
        },
        _ = tx_manager_handle => {
            error!("Tx Manager ended unexpectedly");
        },
    }

    Ok(())
}

/// Subscribes and streams new blocks from WorldChain Sepolia & Prepare CTF Transactions for submission
async fn subscribe_and_prepare<P>(
    provider: Arc<P>,
    identity: Identity,
    private_key: String,
) -> Result<Pin<Box<dyn Stream<Item = Result<Option<Bytes>>> + Send>>>
where
    P: Provider + 'static,
{
    // Fetch the game start & game end
    let pbh_koth = IPBHKotHInstance::new(PBH_CTF_CONTRACT, provider.clone());
    let game_start = pbh_koth.latestBlock().call().await?._0;
    let game_end = pbh_koth.gameEnd().call().await?._0;

    // Fetch the pbh nonce limit
    let pbh_entrypoint = IPBHEntryPointInstance::new(PBH_ENTRY_POINT, provider.clone());
    let pbh_nonce_limit = pbh_entrypoint.numPbhPerMonth().call().await?._0;

    let signer = private_key.parse::<PrivateKeySigner>()?;
    let block_stream = provider.subscribe_blocks().await?.into_stream();

    Ok(Box::pin(stream! {
        tokio::pin!(block_stream);
        let wallet_nonce = provider.get_transaction_count(signer.address()).await?;
        let mut ctf_tx_builder = CtfTransactionBuilder::new(signer, wallet_nonce, pbh_nonce_limit, identity)?;
        info!(game_start = ?game_start, game_end = ?game_end, "Subscribed to Blocks");
        while let Some(header) = block_stream.next().await {
            info!(block_number = ?header.number, "New Block");
            if header.number > game_end.to() || header.number < game_start.to() {
                yield Ok(None);
            }

            yield ctf_tx_builder.prepare_ctf_tx().await;
        }
    }))
}

pub struct CtfTransactionBuilder {
    signer: PrivateKeySigner,
    wallet_nonce: u64,
    pbh_nonce: u16,
    pbh_nonce_limit: u16,
    identity: Identity,
}

impl CtfTransactionBuilder {
    pub fn new(
        signer: PrivateKeySigner,
        wallet_nonce: u64,
        pbh_nonce_limit: u16,
        identity: Identity,
    ) -> Result<Self> {
        Ok(Self {
            signer,
            wallet_nonce,
            pbh_nonce_limit,
            pbh_nonce: 0,
            identity,
        })
    }

    async fn prepare_ctf_tx(&mut self) -> Result<Option<Bytes>> {
        info!("Preparing CTF Transaction");
        let ctf_transaction = if self.pbh_nonce >= self.pbh_nonce_limit {
            ctf_transaction_builder()
                .nonce(self.wallet_nonce)
                .signer(self.signer.clone())
                .call()
                .await?
        } else {
            let tx = pbh_ctf_transaction_builder()
                .nonce(self.wallet_nonce)
                .pbh_nonce(self.pbh_nonce)
                .signer(self.signer.clone())
                .identity(self.identity.clone())
                .call()
                .await?;
            self.pbh_nonce += 1;
            tx
        };

        self.wallet_nonce += 1;
        Ok(Some(ctf_transaction.encoded_2718().into()))
    }
}

pub struct TxManager<P> {
    receiver: tokio::sync::mpsc::Receiver<Bytes>,
    provider: P,
}

impl<P> TxManager<P>
where
    P: Provider,
{
    pub async fn run(mut self) -> Result<()> {
        while let Some(tx) = self.receiver.recv().await {
            let builder = self.provider.send_raw_transaction(&tx).await.map_err(|e| {
                error!(error = ?e, "Error sending transaction");
                e
            })?;

            let receipt = builder.get_receipt().await.map_err(|e| {
                error!(error = ?e, "Error getting receipt");
                e
            })?;
            info!(hash = %receipt.transaction_hash, "Receipt received for Transaction");
        }

        Ok(())
    }
}
