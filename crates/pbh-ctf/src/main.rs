//! PBH CTF starter bot

use std::{path::PathBuf, pin::Pin, sync::Arc};

use alloy_eips::eip2718::Encodable2718;
use alloy_primitives::Bytes;
use alloy_provider::{Provider, ProviderBuilder};
use alloy_signer_local::PrivateKeySigner;
use async_stream::stream;
use config::CtfConfig;
use eyre::eyre::Result;
use futures::{Stream, StreamExt};
use pbh_helpers::{
    Identity, PBH_CTF_CONTRACT, bindings::IPBHKotH::IPBHKotHInstance, ctf_transaction_builder,
    pbh_ctf_transaction_builder,
};
use tracing::info;

mod config;

#[tokio::main]
async fn main() -> Result<()> {
    let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config.toml");
    let config = CtfConfig::load(Some(config_path.as_path()))?;
    let identity = pbh_helpers::derive_identity(&config.secret)?;

    let provider = Arc::new(ProviderBuilder::new().on_http(config.provider.clone()));

    let mut tx_stream = subscribe_and_prepare(
        provider.clone(),
        identity,
        config.private_key,
        config.pbh_nonce_limit,
    )
    .await?;

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
        _ = stream_fut => {},
        _ = tx_manager_handle => {},
    }

    Ok(())
}

/// Subscribes and streams new blocks from WorldChain Sepolia & Prepare CTF Transactions for submission
async fn subscribe_and_prepare<P>(
    provider: Arc<P>,
    identity: Identity,
    private_key: String,
    pbh_nonce_limit: u8,
) -> Result<Pin<Box<dyn Stream<Item = Result<Option<Bytes>>> + Send>>>
where
    P: Provider + 'static,
{
    // Fetch the game start & game end
    let pbh_koth = IPBHKotHInstance::new(PBH_CTF_CONTRACT, provider.clone());
    let game_start = pbh_koth.latestBlock().call().await?._0;
    let game_end = pbh_koth.gameEnd().call().await?._0;

    let signer = private_key.parse::<PrivateKeySigner>()?;
    let block_stream = provider.subscribe_blocks().await?.into_stream();

    Ok(Box::pin(stream! {
        tokio::pin!(block_stream);
        let wallet_nonce = provider.get_transaction_count(signer.address()).await?;
        let mut ctf_tx_builder = CtfTransactionBuilder::new(signer, wallet_nonce, pbh_nonce_limit, identity)?;

        while let Some(header) = block_stream.next().await {
            if header.timestamp > game_end as u64 || header.timestamp < game_start as u64 {
                yield Ok(None);
            }

            yield ctf_tx_builder.prepare_ctf_tx().await;
        }
    }))
}

pub struct CtfTransactionBuilder {
    signer: PrivateKeySigner,
    wallet_nonce: u64,
    pbh_nonce: u8,
    pbh_nonce_limit: u8,
    identity: Identity,
}

impl CtfTransactionBuilder {
    pub fn new(
        signer: PrivateKeySigner,
        wallet_nonce: u64,
        pbh_nonce_limit: u8,
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
        let ctf_transaction = if self.pbh_nonce >= self.pbh_nonce_limit {
            ctf_transaction_builder()
                .nonce(self.wallet_nonce)
                .signer(self.signer.clone())
                .call()
                .await?
        } else {
            let tx = pbh_ctf_transaction_builder()
                .nonce(self.wallet_nonce)
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
            let builder = self.provider.send_raw_transaction(&tx).await?;
            let receipt = builder.get_receipt().await?;
            info!(hash = %receipt.transaction_hash, "Receipt received for Transaction")
        }

        Ok(())
    }
}
