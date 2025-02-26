use std::ops::{Deref, DerefMut};

use alloy_consensus::{TxEnvelope, TypedTransaction};
use alloy_network::{EthereumWallet, TransactionBuilder};
use alloy_primitives::{Address, Bytes, TxKind};
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
    bindings::{
        IMulticall3::{self, Call3},
        IPBHEntryPoint,
    },
};

use crate::{INCLUSION_PROOF_URL, PBH_ENTRY_POINT};
use crate::{PBH_CTF_CONTRACT, PBHProof};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InclusionProof {
    pub root: Field,
    pub proof: semaphore_rs::poseidon_tree::Proof,
}

#[derive(Debug, Clone, Default)]
pub struct CTFTransactionBuilder(pub TransactionRequest);

impl Deref for CTFTransactionBuilder {
    type Target = TransactionRequest;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CTFTransactionBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl CTFTransactionBuilder {
    pub fn new() -> Self {
        let tx = TransactionRequest::default()
            .gas_limit(130000)
            .max_fee_per_gas(1e8 as u128)
            .max_priority_fee_per_gas(1e8 as u128)
            .with_chain_id(WC_SEPOLIA_CHAIN_ID);

        CTFTransactionBuilder(tx)
    }

    pub async fn with_pbh_multicall(
        self,
        identity: &Identity,
        pbh_nonce: u16,
        calls: Vec<Call3>,
    ) -> Result<Self> {
        // Get the inclusion proof for the identity in the from the World Tree
        let proof = fetch_inclusion_proof(&identity).await?;

        // Create the external nullifier hash
        let date = chrono::Utc::now().naive_utc().date();
        let date_marker = DateMarker::from(date);
        let external_nullifier = ExternalNullifier::with_date_marker(date_marker, pbh_nonce);
        let external_nullifier_hash = EncodedExternalNullifier::from(external_nullifier).0;

        let Some(from) = self.0.from else {
            todo!("TODO: handle error");
        };

        let signal_hash = hash_to_field(&SolValue::abi_encode_packed(&(from, calls.clone())));

        let root = proof.root;
        let semaphore_proof = semaphore_rs::protocol::generate_proof(
            &identity,
            &proof.proof,
            external_nullifier_hash,
            signal_hash,
        )?;

        let nullifier_hash =
            semaphore_rs::protocol::generate_nullifier_hash(&identity, external_nullifier_hash);

        //TODO: add helper functions to create the PBH payload
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

        Ok(Self(self.0.input(TransactionInput::new(
            calldata.abi_encode().into(),
        ))))
    }

    pub async fn build(self, signer: PrivateKeySigner) -> Result<TxEnvelope> {
        Ok(self.0.build::<EthereumWallet>(&signer.into()).await?)
    }
}

/// Generates calldata for the PBH King of the Hill game, where the provided player
/// earns a point if they successfully capture the flag.
pub fn king_of_the_hill_calldata(player: Address) -> Bytes {
    crate::bindings::IPBHKotH::ctfCall { receiver: player }
        .abi_encode()
        .into()
}

/// Generates a multicall call, targeting the PBH King of the hill contract.
/// The provided player will earn a point if they successfully capture the flag.
pub fn king_of_the_hill_multicall(player: Address) -> Vec<Call3> {
    let call = IMulticall3::Call3 {
        target: PBH_CTF_CONTRACT,
        callData: crate::bindings::IPBHKotH::ctfCall { receiver: player }
            .abi_encode()
            .into(),
        allowFailure: false,
    };

    vec![call]
}

/// Builds and signs a CTF transaction.
#[builder]
pub async fn pbh_ctf_transaction_builder(
    signer: PrivateKeySigner,
    identity: Identity,
    #[builder(default = 0)] pbh_nonce: u16,
    #[builder(default = 0)] nonce: u64,
    #[builder(default = 130000)] gas_limit: u64,
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
    #[builder(default = 50000)] gas_limit: u64,
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
