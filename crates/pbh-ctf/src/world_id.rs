use base64::{Engine, prelude::BASE64_STANDARD};
use semaphore_rs::{Field, identity::Identity};
use serde::{Deserialize, Serialize};

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InclusionProof {
    pub root: Field,
    pub proof: semaphore_rs::poseidon_tree::Proof,
}
