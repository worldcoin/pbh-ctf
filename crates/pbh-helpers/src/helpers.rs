use alloy_consensus::TxEnvelope;
use alloy_network::{EthereumWallet, TransactionBuilder};
use alloy_primitives::TxKind;
use alloy_rpc_types_eth::{TransactionInput, TransactionRequest};
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_types::{SolCall, SolValue};
use base64::prelude::*;
use bon::builder;
use eyre::Result;
use semaphore_rs::{Field, hash_to_field, identity::Identity};
use serde::{Deserialize, Serialize};
use world_chain_builder_pbh::{
    date_marker::DateMarker,
    external_nullifier::{EncodedExternalNullifier, ExternalNullifier},
    payload::PBHPayload,
};
use world_chain_builder_test_utils::{
    WC_SEPOLIA_CHAIN_ID,
    bindings::{IMulticall3, IPBHEntryPoint},
};

use crate::{INCLUSION_PROOF_URL, PBH_ENTRY_POINT};
use crate::{PBH_CTF_CONTRACT, PBHProof};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InclusionProof {
    pub root: Field,
    pub proof: semaphore_rs::poseidon_tree::Proof,
}

/// Builds and signs a CTF transaction.
#[builder]
pub async fn pbh_ctf_transaction_builder(
    signer: PrivateKeySigner,
    identity: Identity,
    #[builder(default = 0)] pbh_nonce: u16,
    #[builder(default = 0)] nonce: u64,
    #[builder(default = 200000)] gas_limit: u64,
    #[builder(default = 1e8 as u128)] max_fee_per_gas: u128,
    #[builder(default = 1e8 as u128)] max_priority_fee_per_gas: u128,
    #[builder(default = WC_SEPOLIA_CHAIN_ID)] chain_id: u64,
) -> Result<TxEnvelope> {
    let proof = fetch_inclusion_proof(&identity).await?;

    let date = chrono::Utc::now().naive_utc().date();
    let date_marker = DateMarker::from(date);

    let external_nullifier = ExternalNullifier::with_date_marker(date_marker, pbh_nonce);
    let external_nullifier_hash = EncodedExternalNullifier::from(external_nullifier).0;

    let call = IMulticall3::Call3 {
        target: PBH_CTF_CONTRACT,
        callData: crate::bindings::IPBHKotH::ctfCall {
            receiver: signer.address(),
        }
        .abi_encode()
        .into(),
        allowFailure: false,
    };

    let calls = vec![call];

    let signal_hash = hash_to_field(&SolValue::abi_encode_packed(&(
        signer.address(),
        calls.clone(),
    )));

    let root = proof.root;

    let semaphore_proof = semaphore_rs::protocol::generate_proof(
        &identity,
        &proof.proof,
        external_nullifier_hash,
        signal_hash,
    )?;

    let nullifier_hash =
        semaphore_rs::protocol::generate_nullifier_hash(&identity, external_nullifier_hash);

    let payload = PBHPayload {
        root,
        nullifier_hash,
        external_nullifier,
        proof: PBHProof(semaphore_proof),
    };

    let calldata = IPBHEntryPoint::pbhMulticallCall {
        calls,
        payload: payload.into(),
    };

    let tx = TransactionRequest {
        nonce: Some(nonce),
        value: None,
        to: Some(TxKind::Call(PBH_ENTRY_POINT)),
        gas: Some(gas_limit),
        max_fee_per_gas: Some(max_fee_per_gas),
        max_priority_fee_per_gas: Some(max_priority_fee_per_gas),
        chain_id: Some(chain_id),
        input: TransactionInput {
            input: None,
            data: Some(calldata.abi_encode().into()),
        },
        from: Some(signer.address()),
        ..Default::default()
    };

    Ok(tx.build::<EthereumWallet>(&signer.into()).await?)
}

/// Builds and signs a CTF transaction.
#[builder]
pub async fn ctf_transaction_builder(
    signer: PrivateKeySigner,
    #[builder(default = 0)] nonce: u64,
    #[builder(default = 100000)] gas_limit: u64,
    #[builder(default = 1e8 as u128)] max_fee_per_gas: u128,
    #[builder(default = 1e8 as u128)] max_priority_fee_per_gas: u128,
    #[builder(default = WC_SEPOLIA_CHAIN_ID)] chain_id: u64,
) -> Result<TxEnvelope> {
    let tx = TransactionRequest {
        nonce: Some(nonce),
        value: None,
        to: Some(TxKind::Call(PBH_CTF_CONTRACT)),
        gas: Some(gas_limit),
        max_fee_per_gas: Some(max_fee_per_gas),
        max_priority_fee_per_gas: Some(max_priority_fee_per_gas),
        chain_id: Some(chain_id),
        input: TransactionInput {
            input: None,
            data: Some(
                crate::bindings::IPBHKotH::ctfCall {
                    receiver: signer.address(),
                }
                .abi_encode()
                .into(),
            ),
        },
        from: Some(signer.address()),
        ..Default::default()
    };

    Ok(tx.build::<EthereumWallet>(&signer.into()).await?)
}

/// Derive the Semaphore Identity from the a given secret key.
pub fn derive_identity(secret: &str) -> Result<Identity> {
    let decoded = BASE64_STANDARD.decode(secret)?;

    debug_assert_eq!(decoded.len(), 64, "Invalid identity length");

    let trapdoor = &decoded[..32];
    let nullifier = &decoded[32..];

    Ok(Identity {
        trapdoor: Field::from_be_slice(trapdoor),
        nullifier: Field::from_be_slice(nullifier),
    })
}

/// Fetch's a merkle inclusion proof from the Signup Sequencer for a given identity.
pub async fn fetch_inclusion_proof(identity: &Identity) -> Result<InclusionProof> {
    let client = reqwest::Client::new();

    let commitment = identity.commitment();
    let response = client
        .post(format!("{}/inclusionProof", INCLUSION_PROOF_URL))
        .json(&serde_json::json! {{
            "identityCommitment": commitment,
        }})
        .send()
        .await?
        .error_for_status()?;

    let proof: InclusionProof = response.json().await?;

    Ok(proof)
}
