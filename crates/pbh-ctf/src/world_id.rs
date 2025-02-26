use base64::{Engine, prelude::BASE64_STANDARD};
use semaphore_rs::{Field, identity::Identity, protocol::Proof};
use serde::{Deserialize, Serialize};
use world_chain_builder_pbh::{
    date_marker::DateMarker,
    external_nullifier::{EncodedExternalNullifier, ExternalNullifier},
    payload::PBHPayload,
};

use crate::INCLUSION_PROOF_URL;

pub struct WorldID {
    pub identity: Identity,
}

impl WorldID {
    pub fn new(secret: &str) -> eyre::Result<Self> {
        let decoded = BASE64_STANDARD.decode(secret)?;

        debug_assert_eq!(decoded.len(), 64, "Invalid identity length");

        let trapdoor = &decoded[..32];
        let nullifier = &decoded[32..];

        let identity = Identity {
            trapdoor: Field::from_be_slice(trapdoor),
            nullifier: Field::from_be_slice(nullifier),
        };

        Ok(Self { identity })
    }

    pub fn identity(&self) -> &Identity {
        &self.identity
    }

    pub async fn inclusion_proof(&self) -> eyre::Result<InclusionProof> {
        let client = reqwest::Client::new();

        let commitment = self.identity.commitment();
        let response = client
            .post(format!("{}/inclusionProof", INCLUSION_PROOF_URL))
            .json(&serde_json::json! {{
                "identityCommitment": commitment,
            }})
            .send()
            .await?
            .error_for_status()?;

        let proof = response.json().await?;

        Ok(proof)
    }

    /// Generates a PBH external nullifier
    /// Returns `external_nullifier`, `external_nullifier_hash``, `nullifier_hash`
    pub fn pbh_ext_nullifier(&self, pbh_nonce: u16) -> (ExternalNullifier, Field, Field) {
        let date = chrono::Utc::now().naive_utc().date();
        let date_marker = DateMarker::from(date);
        let external_nullifier = ExternalNullifier::with_date_marker(date_marker, pbh_nonce);
        let external_nullifier_hash = EncodedExternalNullifier::from(external_nullifier).0;
        let nullifier_hash = semaphore_rs::protocol::generate_nullifier_hash(
            self.identity(),
            external_nullifier_hash,
        );

        (external_nullifier, external_nullifier_hash, nullifier_hash)
    }

    /// Generates a semaphore proof
    /// Returns the proof and the root of the merkle tree
    /// containing the identity commitments in the set
    pub async fn generate_proof(
        &self,
        signal_hash: Field,
        external_nullifier_hash: Field,
    ) -> eyre::Result<(Proof, Field)> {
        let inclusion_proof = self.inclusion_proof().await?;
        let semaphore_proof = semaphore_rs::protocol::generate_proof(
            self.identity(),
            &inclusion_proof.proof,
            external_nullifier_hash,
            signal_hash,
        )?;

        Ok((semaphore_proof, inclusion_proof.root))
    }

    pub async fn pbh_payload(
        &self,
        pbh_nonce: u16,
        signal_hash: Field,
    ) -> eyre::Result<PBHPayload> {
        let (external_nullifier, external_nullifier_hash, nullifier_hash) =
            self.pbh_ext_nullifier(pbh_nonce);

        let (proof, root) = self
            .generate_proof(signal_hash, external_nullifier_hash)
            .await?;

        let payload = PBHPayload {
            root: root,
            nullifier_hash,
            external_nullifier,
            proof: world_chain_builder_pbh::payload::Proof(proof),
        };

        Ok(payload)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InclusionProof {
    pub root: Field,
    pub proof: semaphore_rs::poseidon_tree::Proof,
}
