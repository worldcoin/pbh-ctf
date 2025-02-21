use alloy_network::{EthereumWallet, TransactionBuilder};
use alloy_primitives::TxKind;
use alloy_rpc_types_eth::{TransactionInput, TransactionRequest};
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_types::SolCall;
use alloy_sol_types::SolValue;
use base64::{Engine, engine::general_purpose};
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
    // The semaphore secret
    let secret = std::env::var("SECRET")?;
    // Derive the Identity from the secret
    let identity = pbh_helpers::derive_identity(&general_purpose::STANDARD.encode(&secret))?;

    // Fetch the merkle inclusion proof for the identity
    let proof = pbh_helpers::fetch_inclusion_proof(&identity).await?;

    // Grab the current data
    let date = chrono::Utc::now().naive_utc().date();
    let date_marker = DateMarker::from(date);

    // Encode the external nullifier from the DateMarker & the nonce
    // The external nullifier must be unique for each transaction sent to the `PBHEntryPoint`
    // The nonce is a u8 in range 0-29
    let external_nullifier = ExternalNullifier::with_date_marker(date_marker, 0);
    let external_nullifier_hash = EncodedExternalNullifier::from(external_nullifier).0;

    let call = IMulticall3::Call3::default();
    let calls = vec![call];

    // Compute the signal hash
    let signal_hash = hash_to_field(&SolValue::abi_encode_packed(&(
        signer.address(),
        calls.clone(),
    )));

    let root = proof.root;

    // Create the Semaphore ZKP.
    let semaphore_proof = semaphore_rs::protocol::generate_proof(
        &identity,
        &proof.proof,
        external_nullifier_hash,
        signal_hash,
    )?;

    // Generate the nullifier hash from the `EncodedExternalNullifier` and `Identity`
    let nullifier_hash =
        semaphore_rs::protocol::generate_nullifier_hash(&identity, external_nullifier_hash);

    // Construct the payload
    let payload = PBHPayload {
        root,
        nullifier_hash,
        external_nullifier,
        proof: PBHProof(semaphore_proof),
    };

    // Construct the transaction data
    let calldata = IPBHEntryPoint::pbhMulticallCall {
        calls,
        payload: payload.into(),
    };

    // Construct the transaction request
    let tx = TransactionRequest {
        nonce: Some(0),
        value: None,
        to: Some(TxKind::Call(PBH_ENTRY_POINT)),
        gas: Some(100000),
        max_fee_per_gas: Some(20e10 as u128),
        max_priority_fee_per_gas: Some(20e10 as u128),
        chain_id: Some(WC_SEPOLIA_CHAIN_ID),
        input: TransactionInput {
            input: None,
            data: Some(calldata.abi_encode().into()),
        },
        from: Some(signer.address()),
        ..Default::default()
    };

    // Build and sign the transaction
    let _signed = tx.build::<EthereumWallet>(&signer.into()).await?;

    Ok(())
}
