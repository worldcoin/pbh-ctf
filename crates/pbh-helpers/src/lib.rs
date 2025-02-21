//! This crate provides helpers for Generating provable transactions on World Chain.
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

pub mod helpers;

/// The URL for the inclusion proof endpoint.
pub const INCLUSION_PROOF_URL: &str = "https://signup-orb-ethereum.stage-crypto.worldcoin.dev";

// Re-exports
pub use helpers::*;

pub use world_chain_builder_pbh::{
    date_marker::DateMarker,
    external_nullifier::{EncodedExternalNullifier, ExternalNullifier},
    payload::{PBHPayload, Proof as PBHProof},
};

pub use world_chain_builder_pool::bindings::*;

pub use semaphore_rs::{
    Field, hash_to_field,
    identity::Identity,
    poseidon_tree::Proof,
    protocol::{generate_nullifier_hash, generate_proof},
};
