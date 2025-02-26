use alloy_network::{EthereumWallet, TransactionBuilder};
use alloy_primitives::Address;
use alloy_primitives::TxKind;
use alloy_rpc_types_eth::{TransactionInput, TransactionRequest};
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_types::SolCall;
use alloy_sol_types::SolValue;
use base64::{Engine, engine::general_purpose};
use pbh_helpers::CTFTransactionBuilder;
use pbh_helpers::CtfTransactionBuilderBuilder;
use pbh_helpers::PBH_CTF_CONTRACT;
use pbh_helpers::king_of_the_hill_calldata;
use pbh_helpers::{
    DateMarker, EncodedExternalNullifier, ExternalNullifier, PBH_ENTRY_POINT, PBHProof,
    hash_to_field,
};
use world_chain_builder_pbh::payload::PBHPayload;
use world_chain_builder_test_utils::{
    WC_SEPOLIA_CHAIN_ID,
    bindings::{IMulticall3, IPBHEntryPoint},
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // The private key of the transaction signer
    let signer = std::env::var("PRIVATE_KEY")?.parse::<PrivateKeySigner>()?;

    let player = Address::random();
    let calldata = king_of_the_hill_calldata(player);

    // The CTF Transaction Builder implements the builder pattern for creating a CTF transaction.
    // All `TransactionRequest` functions are accsessible through this builder
    let tx = CTFTransactionBuilder::new()
        .to(PBH_CTF_CONTRACT)
        .input(calldata.into())
        .build(&signer)
        .await?;

    Ok(())
}
