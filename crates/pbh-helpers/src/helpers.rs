use base64::prelude::*;
use eyre::Result;
use semaphore_rs::{Field, identity::Identity};
use serde::{Deserialize, Serialize};

use crate::INCLUSION_PROOF_URL;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InclusionProof {
    pub root: Field,
    pub proof: semaphore_rs::poseidon_tree::Proof,
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
